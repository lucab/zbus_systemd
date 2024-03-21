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
    println!("!!! NOTE: This example requires root privileges to run.");

    let conn = zbus::Connection::system().await?;
    let manager = zbus_systemd::login1::ManagerProxy::new(&conn).await?;

    let is_enabled = manager.enable_wall_messages().await?;
    let status = if is_enabled { "enabled" } else { "disabled" };
    println!("Shutdown wall messages are currently {status}.");
    if is_enabled {
        let msg = manager.wall_message().await?;
        println!("  -> Current shutdown message: '{msg}'");
    }

    if !is_enabled {
        manager.set_property_enable_wall_messages(true).await?;
        println!("Enabled shutdown wall messages");
    }

    let msg = "Hello from Rust!".to_string();
    manager.set_property_wall_message(msg.clone()).await?;
    println!("Updated shutdown message.");
    println!("  -> New shutdown message: '{msg}'");

    Ok(())
}
