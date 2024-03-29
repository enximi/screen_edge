use screen_edge::config::CONFIG;
use screen_edge::start;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    for (position, operation_action) in &*CONFIG {
        for (operation, action) in operation_action {
            println!("{:?} {:?} {:?}", position, operation, action);
        }
    }

    let handle = start();

    handle.await??;

    Ok(())
}
