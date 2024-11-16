use base64::Engine;
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use instant::Instant;
use serde::Deserialize;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;

#[derive(Debug, Deserialize)]
pub struct SubscriptionResponse {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub payload: Option<serde_json::Value>,
}

pub struct AppSyncWebSocket {
    subscription_id: String,
    writer: Rc<RefCell<SplitSink<WebSocket, Message>>>,
}

impl AppSyncWebSocket {
    pub fn new(
        endpoint: &str,
        token: &str,
        subscription_query: &str,
        on_message: impl Fn(serde_json::Value) + 'static,
    ) -> Self {
        let api_endpoint = endpoint
            .replace("wss://", "")
            .replace("-realtime-api", "-api")
            .replace("/graphql", "");

        let auth_token = if !token.starts_with("Bearer ") {
            format!("Bearer {}", token)
        } else {
            token.to_string()
        };

        let header = serde_json::json!({
            "Authorization": auth_token,
            "host": api_endpoint
        });

        let payload = serde_json::json!({});

        let header_base64 = base64::engine::general_purpose::STANDARD.encode(header.to_string());
        let payload_base64 = base64::engine::general_purpose::STANDARD.encode(payload.to_string());

        let ws_url = format!(
            "{}?header={}&payload={}",
            endpoint, header_base64, payload_base64
        );

        web_sys::console::log_1(&format!("Connecting to: {}", ws_url).into());

        let ws = WebSocket::open_with_protocol(&ws_url, "graphql-ws")
            .expect("Failed to create WebSocket");

        let subscription_id = uuid::Uuid::new_v4().to_string();

        let connection_init = serde_json::json!({
            "type": "connection_init"
        });

        let init_msg = Message::Text(serde_json::to_string(&connection_init).unwrap());

        let subscription_query_json = serde_json::json!({
            "query": subscription_query,
            "variables": {}
        });

        let start_subscription = serde_json::json!({
            "id": subscription_id,
            "type": "start",
            "payload": {
                "data": serde_json::to_string(&subscription_query_json).unwrap(),
                "extensions": {
                    "authorization": {
                        "Authorization": auth_token,
                        "host": api_endpoint
                    }
                }
            }
        });

        let sub_msg = Message::Text(serde_json::to_string(&start_subscription).unwrap());

        let (write, mut read) = ws.split();
        let write = Rc::new(RefCell::new(write));
        let write_clone = write.clone();
        let sub_msg = Rc::new(sub_msg);

        let write_future = async move {
            write_clone.borrow_mut().send(init_msg).await.unwrap();
        };
        spawn_local(write_future);

        let on_message = Rc::new(on_message);
        let on_message_clone = on_message.clone();
        let write = write.clone();
        let write_for_return = write.clone();
        let read_future = async move {
            let last_ka = Rc::new(RefCell::new(Instant::now()));
            let last_ka_clone = last_ka.clone();

            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        web_sys::console::log_1(&format!("Received message: {}", text).into());
                        if let Ok(response) = serde_json::from_str::<SubscriptionResponse>(&text) {
                            match response.msg_type.as_str() {
                                "connection_ack" => {
                                    web_sys::console::log_1(
                                        &"Connection acknowledged, starting subscription".into(),
                                    );
                                    let write = write.clone();
                                    let sub_msg = sub_msg.clone();
                                    spawn_local(async move {
                                        gloo_timers::future::TimeoutFuture::new(100).await;
                                        write.borrow_mut().send((*sub_msg).clone()).await.unwrap();
                                    });
                                    if let Some(payload) = response.payload {
                                        if let Some(timeout_ms) = payload.get("connectionTimeoutMs")
                                        {
                                            let timeout_ms = timeout_ms.as_u64().unwrap_or(300000);
                                            let last_ka = last_ka_clone.clone();

                                            spawn_local(async move {
                                                loop {
                                                    gloo_timers::future::TimeoutFuture::new(1000)
                                                        .await;
                                                    let elapsed =
                                                        last_ka.borrow().elapsed().as_millis();
                                                    if elapsed > timeout_ms as u128 {
                                                        // Close connection if no keepalive received
                                                        break;
                                                    }
                                                }
                                            });
                                        }
                                    }
                                }
                                "start_ack" => {
                                    web_sys::console::log_1(
                                        &"Subscription started successfully".into(),
                                    );
                                }
                                "data" => {
                                    if let Some(payload) = response.payload {
                                        on_message_clone(payload);
                                    }
                                }
                                "error" | "connection_error" => {
                                    web_sys::console::log_1(
                                        &format!("Error: {:?}", response.payload).into(),
                                    );
                                }
                                "ka" => {
                                    web_sys::console::log_1(&"Received keepalive".into());
                                    *last_ka.borrow_mut() = Instant::now();
                                }
                                _ => {
                                    web_sys::console::log_1(
                                        &format!("Unknown message type: {}", response.msg_type)
                                            .into(),
                                    );
                                }
                            }
                        }
                    }
                    Ok(Message::Bytes(_)) => {
                        web_sys::console::log_1(&"Received binary message".into());
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("WebSocket error: {:?}", e).into());
                    }
                }
            }
        };
        spawn_local(read_future);

        Self {
            subscription_id,
            writer: write_for_return,
        }
    }

    pub fn close(&self) {
        let subscription_id = self.subscription_id.clone();
        let writer = self.writer.clone();

        spawn_local(async move {
            let stop_subscription = serde_json::json!({
                "id": subscription_id,
                "type": "stop"
            });

            let stop_msg = Message::Text(serde_json::to_string(&stop_subscription).unwrap());
            writer.borrow_mut().send(stop_msg).await.unwrap();
        });
    }
}

impl Drop for AppSyncWebSocket {
    fn drop(&mut self) {
        self.close();
    }
}
