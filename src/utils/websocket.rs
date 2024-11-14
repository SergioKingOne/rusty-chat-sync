use futures::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;

#[derive(Debug, Serialize)]
struct SubscriptionMessage {
    id: String,
    #[serde(rename = "type")]
    msg_type: String,
    payload: SubscriptionPayload,
}

#[derive(Debug, Serialize)]
struct SubscriptionPayload {
    data: String,
    extensions: Extensions,
}

#[derive(Debug, Serialize)]
struct Extensions {
    authorization: String,
}

#[derive(Debug, Deserialize)]
pub struct SubscriptionResponse {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub payload: Option<serde_json::Value>,
}

pub struct AppSyncWebSocket {
    subscription_id: String,
}

impl AppSyncWebSocket {
    pub fn new(
        endpoint: &str,
        token: &str,
        subscription_query: &str,
        on_message: impl Fn(serde_json::Value) + 'static,
    ) -> Self {
        let ws = WebSocket::open(endpoint).expect("Failed to create WebSocket");
        let subscription_id = uuid::Uuid::new_v4().to_string();

        // Create connection init message
        let connection_init = SubscriptionMessage {
            id: "connection_init".to_string(),
            msg_type: "connection_init".to_string(),
            payload: SubscriptionPayload {
                data: subscription_query.to_string(),
                extensions: Extensions {
                    authorization: token.to_string(),
                },
            },
        };

        let init_msg = Message::Text(serde_json::to_string(&connection_init).unwrap());

        // Create start subscription message
        let start_subscription = SubscriptionMessage {
            id: subscription_id.clone(),
            msg_type: "start".to_string(),
            payload: SubscriptionPayload {
                data: subscription_query.to_string(),
                extensions: Extensions {
                    authorization: token.to_string(),
                },
            },
        };

        let sub_msg = Message::Text(serde_json::to_string(&start_subscription).unwrap());

        // Split websocket for separate read/write
        let (mut write, mut read) = ws.split();

        // Handle sending messages
        let write_future = async move {
            write.send(init_msg).await.unwrap();
            write.send(sub_msg).await.unwrap();
        };
        spawn_local(write_future);

        // Handle receiving messages
        let on_message = Rc::new(on_message);
        let on_message_clone = on_message.clone();
        let read_future = async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Ok(response) = serde_json::from_str::<SubscriptionResponse>(&text) {
                            match response.msg_type.as_str() {
                                "data" => {
                                    if let Some(payload) = response.payload {
                                        on_message_clone(payload);
                                    }
                                }
                                "connection_ack" => {
                                    web_sys::console::log_1(
                                        &"WebSocket connection acknowledged".into(),
                                    );
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        };
        spawn_local(read_future);

        Self {
            subscription_id,
        }
    }

    pub fn close(&self) {
        let subscription_id = self.subscription_id.clone();
        spawn_local(async move {
            if let Ok(mut ws) = WebSocket::open("wss://4psoayuvcnfu7ekadjzgs6erli.appsync-realtime-api.us-east-1.amazonaws.com/graphql") {
                let stop_subscription = SubscriptionMessage {
                    id: subscription_id,
                    msg_type: "stop".to_string(),
                    payload: SubscriptionPayload {
                        data: "".to_string(),
                        extensions: Extensions {
                            authorization: "".to_string(),
                        },
                    },
                };

                let stop_msg = Message::Text(serde_json::to_string(&stop_subscription).unwrap());
                
                // Send stop message before closing
                ws.send(stop_msg).await.unwrap();
                ws.close(Some(1000), Some("Client disconnecting")).unwrap();
            }
        });
    }
}

impl Drop for AppSyncWebSocket {
    fn drop(&mut self) {
        self.close();
    }
}
