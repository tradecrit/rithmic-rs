use std::collections::HashMap;

use tokio::sync::oneshot;
use tracing::{event, Level};

use crate::{api::receiver_api::RithmicResponse, rti::messages::RithmicMessage};
use crate::rti::depth_by_order::TransactionType;

#[derive(Debug)]
pub struct RithmicRequest {
    pub request_id: String,
    pub responder: oneshot::Sender<Result<Vec<RithmicResponse>, String>>,
}

#[derive(Debug)]
pub struct RithmicRequestHandler {
    handle_map: HashMap<String, oneshot::Sender<Result<Vec<RithmicResponse>, String>>>,
    response_vec_map: HashMap<String, Vec<RithmicResponse>>,
    snapshot_side_map: HashMap<String, (bool, bool)>, // (has_bid, has_ask)
}

#[derive(Default)]
struct SnapshotAccumulator {
    responses: Vec<RithmicResponse>,
    has_bid: bool,
    has_ask: bool,
}


impl RithmicRequestHandler {
    pub fn new() -> Self {
        Self {
            handle_map: HashMap::new(),
            response_vec_map: HashMap::new(),
            snapshot_side_map: HashMap::new(),
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
                if !response.multi_response {
                    if let Some(responder) = self.handle_map.remove(&response.request_id) {
                        let _ = responder.send(Ok(vec![response]));
                    } else {
                        event!(Level::ERROR, "No responder found for response: {:#?}", response);
                    }
                    return;
                }

                // Accumulate response part
                let responses = self
                    .response_vec_map
                    .entry(response.request_id.clone())
                    .or_default();

                responses.push(response.clone());

                // Update snapshot-side tracking if applicable
                if let RithmicMessage::ResponseDepthByOrderSnapshot(snap) = &response.message {
                    let entry = self
                        .snapshot_side_map
                        .entry(response.request_id.clone())
                        .or_insert((false, false));

                    let transaction_type = snap.depth_side.as_ref().and_then(|side| {
                        TransactionType::try_from(*side).ok()
                    });

                    match transaction_type {
                        Some(TransactionType::Buy) => entry.0 = true, // has_bid
                        Some(TransactionType::Sell) => entry.1 = true, // has_ask
                        None => {
                            event!(Level::WARN, "Invalid or missing depth side in snapshot: {:?}", snap);
                        }
                    }
                }

                // Final part
                if !response.has_more {
                    let can_send = match &response.message {
                        RithmicMessage::ResponseDepthByOrderSnapshot(_) => {
                            matches!(self.snapshot_side_map.get(&response.request_id), Some((true, true)))
                        }
                        _ => true, // Non-snapshot messages are not delayed
                    };

                    if can_send {
                        if let Some(responder) = self.handle_map.remove(&response.request_id) {
                            let responses = self.response_vec_map.remove(&response.request_id).unwrap_or_default();
                            let _ = responder.send(Ok(responses));
                            self.snapshot_side_map.remove(&response.request_id);
                        } else {
                            event!(Level::ERROR, "No responder found for response: {:#?}", response);
                        }
                    } else {
                        event!(Level::WARN, "Delaying response for snapshot until both sides received: {:?}", response.request_id);
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
