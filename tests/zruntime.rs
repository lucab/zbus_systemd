#![cfg(all(unix, feature = "systemd1", feature = "zbus-async-zruntime"))]

use std::{net::Shutdown, os::unix::net::UnixStream, sync::mpsc, time::Duration};
use zbus_systemd::{systemd1::ManagerProxy, zbus};

struct Manager;

#[zbus::interface(name = "org.freedesktop.systemd1.Manager")]
impl Manager {
    fn get_unit(&self, _name: String) -> zbus::OwnedObjectPath {
        zbus::OwnedObjectPath::try_from("/org/freedesktop/systemd1/unit/qualification")
            .expect("unit path")
    }
}

#[test]
fn generated_proxy_makes_progress_on_zruntime() {
    let (server, client) = UnixStream::pair().expect("private socket pair");
    let deadline_socket = client.try_clone().expect("deadline socket");
    let (cancel_deadline, deadline) = mpsc::channel();
    let deadline_thread = std::thread::spawn(move || {
        if deadline.recv_timeout(Duration::from_secs(10)).is_err() {
            let _ = deadline_socket.shutdown(Shutdown::Both);
        }
    });
    let server = std::thread::spawn(move || {
        zruntime::block_on(
            zbus::connection::Builder::unix_stream(server)
                .server(zbus::Guid::generate())
                .p2p()
                .serve_at("/org/freedesktop/systemd1", Manager)
                .build(),
        )
        .expect("private ZRuntime server")
    });
    // There is no Tokio runtime on either thread. With the explicitly named
    // feature, both connections must use the built-in ZRuntime instead.
    // zbus::block_on selects Tokio when both runtime features are enabled.
    // Drive ZRuntime explicitly so this test covers it under --all-features too.
    zruntime::block_on(async {
        let client = zbus::connection::Builder::unix_stream(client)
            .p2p()
            .build()
            .await
            .expect("private ZRuntime client");
        let server = server.join().expect("server thread");
        let manager = ManagerProxy::new(&client).await.expect("manager proxy");
        let path = manager
            .get_unit("qualification.service".to_owned())
            .await
            .expect("GetUnit makes progress");
        assert_eq!(
            path.as_str(),
            "/org/freedesktop/systemd1/unit/qualification"
        );
        server.close().await.expect("close owned server");
        assert!(manager
            .get_unit("qualification.service".to_owned())
            .await
            .is_err());
        drop(manager);
        client.close().await.expect("close owned client");
    });
    cancel_deadline.send(()).expect("cancel test deadline");
    deadline_thread.join().expect("deadline thread");
}
