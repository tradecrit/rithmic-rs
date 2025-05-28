use std::collections::HashMap;
use tokio::sync::oneshot;
use tracing::{event, Level};

use crate::{api::receiver_api::RithmicResponse, rti::messages::RithmicMessage};

#[derive(Debug)]
pub struct RithmicRequest {
    pub request_id: String,
    pub responder: oneshot::Sender<Result<Vec<RithmicResponse>, String>>,
}

#[derive(Debug)]
pub struct RithmicRequestHandler {
    handle_map: HashMap<String, oneshot::Sender<Result<Vec<RithmicResponse>, String>>>,
    response_vec_map: HashMap<String, Vec<RithmicResponse>>,
    groups_complete: i32,
}

impl RithmicRequestHandler {
    pub fn new() -> Self {
        Self {
            handle_map: HashMap::new(),
            response_vec_map: HashMap::new(),
            groups_complete: 0
        }
    }

    pub fn register_request(&mut self, request: RithmicRequest) {
        self.handle_map
            .insert(request.request_id, request.responder);
    }

    pub fn handle_response(&mut self, response: RithmicResponse) {
        match response.message {
            RithmicMessage::ResponseHeartbeat(_) => {}
            _ => {
                // Case 1: Single-message response
                if !response.multi_response {
                    if let Some(responder) = self.handle_map.remove(&response.request_id) {
                        let _ = responder.send(Ok(vec![response]));
                    } else {
                        event!(Level::ERROR, "No responder found for response: {:#?}", response);
                    }
                    return;
                }


                tracing::info!("Response received: {:#?}", response);

                // Case 2: Multi-message response (grouped)
                let responses = self
                    .response_vec_map
                    .entry(response.request_id.clone())
                    .or_default();

                responses.push(response.clone());

                if !response.has_more {
                    if &self.groups_complete < &response.message_count {
                        tracing::info!("Message count: {}, current complete: {}", response.message_count, self.groups_complete);
                        self.groups_complete += 1;
                    } else {
                        tracing::warn!("Received more messages than expected for request ID: {}", response.request_id);
                    }
                }

                // Only proceed if this was the last message in the group
                if !response.has_more && self.groups_complete == response.message_count {
                    tracing::info!("Received more messages than expected for request ID: {}", response.request_id);

                    // Final group of messages
                    if let Some(responder) = self.handle_map.remove(&response.request_id) {
                        let grouped_responses = self.response_vec_map.remove(&response.request_id)
                            .unwrap_or_else(|| vec![response]);

                        if let Err(e) = responder.send(Ok(grouped_responses)) {
                            event!(Level::ERROR, "Failed to send multi-response: {:?}", e);
                        }
                    } else {
                        event!(Level::ERROR, "No responder found for final multi-response: {:#?}", response);
                    }
                }
            }
        }
    }
}

impl Default for RithmicRequestHandler {
    fn default() -> Self {
        Self::new()
    }
}
