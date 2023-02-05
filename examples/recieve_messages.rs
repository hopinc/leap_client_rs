use std::time::Duration;

use leap_client_rs::{leap::types::Event, LeapEdge, LeapOptions};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut manager = LeapEdge::new(LeapOptions {
        project: &std::env::var("PROJECT").unwrap(),
        token: std::env::var("TOKEN").ok().as_deref(),
        ..Default::default()
    })
    .await?;

    if let Ok(channel) = std::env::var("CHANNEL") {
        manager.channel_subscribe(&channel).await?;
    }

    while let Some(event) = manager.listen().await {
        if matches!(event, Event::Message(_) | Event::DirectMessage(_)) {
            println!("{event:?}");
        }
    }

    sleep(Duration::from_secs(5)).await;

    log::debug!("Done :D");

    Ok(())
}
