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
    let conn = zbus::Connection::system().await?;
    let manager = zbus_systemd::systemd1::ManagerProxy::new(&conn).await?;
    let target = manager.get_default_target().await?;
    println!("Default target: '{}'", target);
    Ok(())
}
