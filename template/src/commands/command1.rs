use miette::Result;
use tokio::time::{sleep, Duration};
use tokio_graceful_shutdown::SubsystemHandle;
use tracing::info;

pub async fn run(subsys: SubsystemHandle) -> Result<()> {
    info!("command1 started.");
    tokio::select! {
        _ = subsys.on_shutdown_requested() => {
            info!("countdown cancelled.");
        },
        _ = countdown() => {
            info!("countdown finished.");
        }
    };
    info!("command1 stopped");

    Ok(())
}

async fn countdown() {
    for i in (1..10).rev() {
        info!("command1 countdown: {}", i);
        sleep(Duration::from_millis(1000)).await;
    }
}
