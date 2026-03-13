use std::sync::Arc;

use axum::extract::ws::{Message as AxumMessage, WebSocket};
use axum::{
    Router, extract::State, extract::WebSocketUpgrade, response::IntoResponse, routing::get,
};
use futures::{SinkExt, stream::StreamExt};
use google_cognitive_apis::api::grpc::google::cloud::speechtotext::v2::StreamingRecognizeRequest;
use google_cognitive_apis::api::grpc::google::cloud::speechtotext::v2::streaming_recognize_request::StreamingRequest;
use loony_cognitive_services::google::GoogleSTT;
use std::env;

// WebSocket handler
async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket: WebSocket| async {
        handle_socket(socket).await;
    })
}

// Function that handles the actual websocket connection
async fn handle_socket(mut socket: WebSocket) {
    let model: String = std::env::var("MODEL").unwrap_or_else(|_| "phone_call".to_string());
    let language: String = std::env::var("LANGUAGE").unwrap_or_else(|_| "en-US".to_string());

    let sample_rate_hertz: i32 = std::env::var("SAMPLE_RATE_HERTZ")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8000);

    let audio_channel_count: i32 = std::env::var("AUDIO_CHANNEL_COUNT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);
    println!(
        "{} {} {} {}",
        model, language, sample_rate_hertz, audio_channel_count
    );

    let app_config = (model, language, sample_rate_hertz, audio_channel_count);

    println!("WebSocket connection established");

    let stt = GoogleSTT::new(app_config).await.unwrap();
    let audio_sink = stt.audio_sink.clone().unwrap();
    while let Some(Ok(msg)) = socket.next().await {
        match msg {
            AxumMessage::Text(text) => {
                if text == "START_VOICE_RECORDING" {
                    log::debug!("START_VOICE_RECORDING");

                    let req = StreamingRecognizeRequest {
                        recognizer: stt.recognizer_id.clone(),
                        streaming_request: Some(StreamingRequest::StreamingConfig(
                            stt.streaming_recognition_config.clone(),
                        )),
                    };
                    audio_sink.send(req).await.unwrap();
                }
                if text == "STOP_VOICE_RECORDING" {
                    log::debug!("STOP_VOICE_RECORDING");
                    break;
                }
            }
            AxumMessage::Binary(bin) => {
                let req = StreamingRecognizeRequest {
                    recognizer: stt.recognizer_id.clone(),
                    streaming_request: Some(StreamingRequest::Audio(bin.to_vec())),
                };
                audio_sink.send(req).await.unwrap();
            }
            _ => {}
        }
    }
    socket.close().await.unwrap();
    println!("Websocket closed.");
}

struct AppState {}

#[tokio::main]
async fn main() {
    env_logger::init();
    let port = std::env::var("PORT").unwrap_or("2000".to_string());
    let port = port.parse::<i32>().unwrap();

    let app_state = AppState {};
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(Arc::new(app_state));
    let url = format!("127.0.0.1:{}", port);

    let listener = tokio::net::TcpListener::bind(&url).await.unwrap();

    log::info!("Listening on {}", url);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
