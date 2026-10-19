#![cfg(all(unix, feature = "systemd1", feature = "zbus-async-tokio"))]

use std::{
    os::unix::net::UnixStream,
    sync::{Arc, Mutex},
    time::Duration,
};
use zbus_systemd::{
    systemd1::{ManagerProxy, UnitProxy},
    wire::{serialized::Context, to_bytes, LE},
    zbus::{self, OwnedObjectPath, OwnedVariant, Type, Variant},
};

type Properties = Vec<(String, OwnedVariant)>;
type Auxiliary = Vec<(String, Properties)>;
type StartCall = (String, String, Properties, Auxiliary);

fn properties() -> Properties {
    let command = vec![(
        "/usr/bin/example-service".to_owned(),
        vec!["example-service".to_owned(), "start".to_owned()],
        false,
    )];
    vec![
        (
            "ExecStart".to_owned(),
            OwnedVariant::try_from(Variant::from(command)).expect("command variant"),
        ),
        (
            "RuntimeDirectory".to_owned(),
            OwnedVariant::try_from(Variant::from(vec!["example-service".to_owned()]))
                .expect("directory variant"),
        ),
        (
            "CPUQuotaPerSecUSec".to_owned(),
            OwnedVariant::from(2_000_000_u64),
        ),
        ("PrivateTmp".to_owned(), OwnedVariant::from(true)),
    ]
}

#[test]
fn transient_unit_properties_roundtrip() {
    type Commands = Vec<(String, Vec<String>, bool)>;
    assert_eq!(Commands::SIGNATURE.to_string(), "a(sasb)");
    assert_eq!(Properties::SIGNATURE.to_string(), "a(sv)");
    assert_eq!(Auxiliary::SIGNATURE.to_string(), "a(sa(sv))");
    let call = (
        "qualification.service".to_owned(),
        "replace".to_owned(),
        properties(),
        Vec::<(String, Properties)>::new(),
    );
    let bytes = to_bytes(Context::new(LE, 0), &call).expect("encode StartTransientUnit body");
    let (decoded, used): (StartCall, usize) =
        bytes.deserialize().expect("decode StartTransientUnit body");
    assert_eq!(decoded, call);
    assert_eq!(used, bytes.len());
}

#[test]
fn serialized_descriptor_outlives_original() {
    let file = std::fs::File::open("/dev/null").expect("owned test descriptor");
    let fd = zbus::OwnedFd::from(std::os::fd::OwnedFd::from(file));
    let bytes = to_bytes(Context::new(LE, 0), &fd).expect("encode descriptor");
    drop(fd);
    let (decoded, _): (zbus::OwnedFd, usize) =
        bytes.deserialize().expect("decode owned descriptor");
    let file = std::fs::File::from(std::os::fd::OwnedFd::from(decoded));
    assert!(file.metadata().is_ok());
}

#[derive(Debug, zbus::DBusError)]
#[zbus(prefix = "org.freedesktop.systemd1")]
enum ManagerError {
    NoSuchUnit(String),
}

struct Manager(Arc<Mutex<Option<StartCall>>>);

#[zbus::interface(name = "org.freedesktop.systemd1.Manager")]
impl Manager {
    fn get_unit(&self, name: &str) -> Result<OwnedObjectPath, ManagerError> {
        if name != "qualification.service" {
            return Err(ManagerError::NoSuchUnit(name.to_owned()));
        }
        Ok(
            OwnedObjectPath::try_from("/org/freedesktop/systemd1/unit/qualification")
                .expect("unit path"),
        )
    }

    fn start_transient_unit(
        &self,
        name: String,
        mode: String,
        properties: Properties,
        aux: Auxiliary,
    ) -> OwnedObjectPath {
        *self.0.lock().expect("capture lock") = Some((name, mode, properties, aux));
        OwnedObjectPath::try_from("/org/freedesktop/systemd1/job/1").expect("job path")
    }

    fn stop_unit(&self, _name: String, _mode: String) -> zbus::fdo::Result<OwnedObjectPath> {
        Err(zbus::fdo::Error::AccessDenied("test refusal".to_owned()))
    }
}

struct Unit;

#[zbus::interface(name = "org.freedesktop.systemd1.Unit")]
impl Unit {
    #[zbus(property)]
    fn active_state(&self) -> String {
        "active".to_owned()
    }
}

#[tokio::test]
async fn generated_proxies_preserve_calls_properties_and_errors() {
    tokio::time::timeout(Duration::from_secs(10), async {
        let captured = Arc::new(Mutex::new(None));
        let (server, client) = UnixStream::pair().expect("private socket pair");
        let server = zbus::connection::Builder::unix_stream(server)
            .server(zbus::Guid::generate()).p2p()
            .serve_at("/org/freedesktop/systemd1", Manager(Arc::clone(&captured)))
            .serve_at("/org/freedesktop/systemd1/unit/qualification", Unit);
        let client = zbus::connection::Builder::unix_stream(client).p2p();
        let (server, client) = tokio::try_join!(server.build(), client.build()).expect("owned private connections");
        let manager = ManagerProxy::new(&client).await.expect("manager proxy");
        let path = manager.get_unit("qualification.service".to_owned()).await.expect("GetUnit");
        let unit = UnitProxy::new(&client, path).await.expect("unit proxy");
        assert_eq!(unit.active_state().await.expect("ActiveState"), "active");
        let expected = ("qualification.service".to_owned(), "replace".to_owned(), properties(), Vec::new());
        let job = manager.start_transient_unit(expected.0.clone(), expected.1.clone(), expected.2.clone(), expected.3.clone()).await.expect("StartTransientUnit");
        assert_eq!(job.as_str(), "/org/freedesktop/systemd1/job/1");
        assert_eq!(captured.lock().expect("capture lock").take(), Some(expected));
        let missing = manager.get_unit("missing.service".to_owned()).await.expect_err("missing unit");
        assert!(matches!(missing, zbus::Error::MethodError(name, _, _) if name.as_str() == "org.freedesktop.systemd1.NoSuchUnit"));
        let denied = manager.stop_unit("qualification.service".to_owned(), "replace".to_owned()).await.expect_err("refused stop");
        assert!(matches!(denied, zbus::Error::FDO(error) if matches!(*error, zbus::fdo::Error::AccessDenied(_))));
        server.close().await.expect("close owned server");
        assert!(manager.get_unit("qualification.service".to_owned()).await.is_err());
        drop(unit);
        drop(manager);
        client.close().await.expect("close owned client");
    }).await.expect("private D-Bus test deadline");
}
