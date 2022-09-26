use zbus_systemd::login1::{ManagerProxy, SessionProxy};

type ExResult<T> = Result<T, Box<dyn std::error::Error + 'static>>;

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let ret = rt.block_on(run());
    if let Err(e) = ret {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> ExResult<()> {
    let session_id = "4";
    let conn = zbus::Connection::system().await?;

    let session_obj_path = {
        let manager = ManagerProxy::new(&conn).await?;
        manager.get_session(session_id.to_string()).await?
    };
    println!("Session object path: {}", session_obj_path.as_str());

    let session = SessionProxy::builder(&conn)
        .path(session_obj_path)?
        .build()
        .await?;

    session.lock().await?;
    println!("Locked");

    session.unlock().await?;
    println!("Unlocked");

    Ok(())
}
