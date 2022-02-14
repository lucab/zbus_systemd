type ExResult<T> = Result<T, Box<dyn std::error::Error + 'static>>;

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let ret = rt.block_on(run());
    if let Err(e) = ret {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

async fn run() -> ExResult<()> {
    let conn = zbus_systemd::connect_system_dbus().await?;
    let timedated = zbus_systemd::timedate1::TimedatedProxy::new(&conn).await?;
    let timezones = timedated.list_timezones().await?;
    if timezones.len() > 0 {
        println!("Timezones:");
        for tz in &timezones {
            println!(" - {}", tz);
        }
    }
    println!("Total timezones: {}", timezones.len());
    Ok(())
}
