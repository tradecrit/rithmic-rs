use std::time::Duration;

use async_trait::async_trait;
use tokio::time::{interval_at, Instant, Interval};
use tokio_tungstenite::tungstenite::{Error, Message};

pub trait RithmicStream {
    type Handle;

    fn get_handle(&self) -> Self::Handle;
}

#[async_trait]
pub trait PlantActor {
    type Command;

    async fn run(&mut self);
    async fn handle_command(&mut self, command: Self::Command);
    async fn handle_rithmic_message(&mut self, message: Result<Message, Error>) -> Result<bool, ()>;
}

pub fn get_heartbeat_interval() -> Interval {
    let heartbeat_interval = Duration::from_secs(60);
    let start_offset = Instant::now() + heartbeat_interval;

    interval_at(start_offset, heartbeat_interval)
}
