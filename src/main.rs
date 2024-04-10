use futures::{SinkExt, StreamExt};
use tokio::time::{interval, Duration};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::connect_async;
use url::Url;
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize)]
struct RegisterRequest {
    machine_id: String
}

// Response body structure for server URL
#[derive(Deserialize)]
struct RegisterResponse {
    url: String,
}


#[tokio::main]
async fn main() {
    // Create a Reqwest HTTP client
    let http_client = Client::new();

    // Prepare the register request body
    let register_request = RegisterRequest {
        machine_id: "ipv6".to_string()
    };

    // Make the register API request
    let register_response: RegisterResponse = http_client
        .post("http://localhost:8000/register")
        .header("Content-Type", "application/json")
        .json(&register_request)
        .send()
        .await
        .expect("Failed to send register request")
        .json()
        .await
        .expect("Failed to parse register response");

    // Parse the WebSocket server URL from the register response
    let url = Url::parse(&register_response.url).expect("Invalid server URL");

    // Connect to the WebSocket server
    let (ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect to server");

    let (mut write, mut read) = ws_stream.split();

    // Send a message to the server
    if let Err(e) = write.send(Message::Text("ping".to_string())).await {
        eprintln!("Failed to send message: {}", e);
    }

    // Heartbeat interval (30 seconds)
    let mut interval = interval(Duration::from_secs(30));

    // Process incoming messages and send heartbeat pings
    while let Some(Ok(msg)) = read.next().await {
        match msg {
            Message::Text(text) => {
                println!("Received message: {}", text);
                // Handle other message types as needed
            }
            _ => {
                // Ignore other message types
            }
        }

        // Send a ping message every 30 seconds
        interval.tick().await;
        if let Err(e) = write.send(Message::Text("ping".to_string())).await {
            eprintln!("Failed to send ping: {}", e);
            return;
        }
    }

    // Process incoming messages
    while let Some(Ok(msg)) = read.next().await {
        match msg {
            Message::Text(text) => {
                println!("Received message: {}", text);
                // Handle other message types as needed
            }
            _ => {
                // Ignore other message types
            }
        }
    }
}
