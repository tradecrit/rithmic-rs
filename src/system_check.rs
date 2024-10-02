use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::{event, Level};

use crate::api::receiver_api::RithmicReceiverApi;
use crate::api::sender_api::RithmicSenderApi;
use crate::connection_info;
use crate::connection_info::AccountInfo;
use crate::rti::messages::RithmicMessage;

pub struct RithmicSystemCheckClient {}

impl RithmicSystemCheckClient {
    pub async fn list_systems<'a>(
        env: &connection_info::RithmicConnectionSystem,
        account_info: &AccountInfo,
    ) -> Result<Vec<String>, ()> {
        let config = connection_info::get_config(env);

        event!(Level::INFO, "Listing Rithmic systems {}", config.url);

        let mut sender_api = RithmicSenderApi::new(account_info);

        let receiver_api = RithmicReceiverApi {
            source: "system_check".to_string(),
        };


        let (ws_stream, _) = connect_async(&config.url).await.expect("Failed to connect");
        let (mut write, mut read) = ws_stream.split();

        let (message_buf, _) = sender_api.request_rithmic_system_info();

        write
            .send(tokio_tungstenite::tungstenite::Message::Binary(message_buf))
            .await
            .expect("Failed to send request");

        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Close(_)) => break,
                Ok(Message::Binary(data)) => {
                    let response = receiver_api.buf_to_message(data).unwrap();

                    let RithmicMessage::ResponseRithmicSystemInfo(msg) = response.message else {
                        panic!("Unexpected message type: {:#?}", response)
                    };

                    return Ok(msg.system_name);
                }
                _ => {
                    match message {
                        Ok(msg) => msg.into_data(),
                        Err(e) => {
                            event!(Level::ERROR, "Error receiving message: {}", e);
                            continue;
                        }
                    };
                }
            }
        }

        Err(())
    }
}
