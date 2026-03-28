use futures_util::StreamExt;
use google_cloud_speech_v2::builder::speech::ClientBuilder;
use google_cloud_speech_v2::client::Speech;
use google_cloud_speech_v2::model::streaming_recognize_request::StreamingRequest;
use google_cloud_speech_v2::model::{
    ExplicitDecodingConfig, RecognitionConfig, StreamingRecognitionConfig,
    StreamingRecognizeRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize the client
    let speech_client = Speech::builder().build().await?;
    let x = speech_client.batch_recognize().poller().untill_done();

    // 2. Define the Recognizer path
    // Replace {PROJECT_ID} and {LOCATION}. Use "_" for a default recognizer.
    let recognizer = "projects/your-project-id/locations/global/recognizers/_";

    let mut config = RecognitionConfig::default();

    config.model = "chirp_2".to_string();
    config.language_codes = vec!["en-US".to_string()];
    config.features = None;
    config.adaptation = None;
    config.transcript_normalization = None;

    let mut stream_config = StreamingRecognitionConfig::default();

    stream_config.config = Some(config);
    stream_config.config_mask = None;
    stream_config.streaming_features = None;

    let mut streaming_request_config =
        StreamingRequest::StreamingConfig(Box::new(stream_config.clone()));
    // 4. Create the request stream
    // The first message MUST contain the streaming_config
    // let first_request = StreamingRecognizeRequest {
    //     recognizer: recognizer.to_string(),
    //     streaming_request: Some(google_cloud_speech_v2::model::streaming_recognize_request::StreamingRequest::StreamingConfig(config)),
    //     ..Default::default()
    // };

    let recognizer = format!("projects/{}/locations/global/recognizers/_", "");

    let mut config_request = StreamingRecognizeRequest::default();
    config_request.recognizer = recognizer.clone();
    config_request.streaming_request = Some(streaming_request_config);

    let mut streaming_request_audio =
        StreamingRequest::Audio(tokio_tungstenite::tungstenite::Bytes::new());

    // In a real app, you'd use a channel to pipe microphone data here
    let audio_data = vec![0u8; 3200]; // Dummy 100ms of silence
    let mut audio_request = StreamingRecognizeRequest::default();
    audio_request.recognizer = recognizer;
    audio_request.streaming_request = Some(streaming_request_audio);

    let requests = futures_util::stream::iter(vec![config_request, audio_request]);

    // 5. Start streaming
    // let mut response_stream = client.streaming_recognize(requests, None).await?;

    // println!("Listening for results...");
    // while let Some(response) = response_stream.next().await {
    //     let resp = response?;
    //     for result in resp.results {
    //         if let Some(alt) = result.alternatives.first() {
    //             println!(
    //                 "Transcript: {} (Final: {})",
    //                 alt.transcript, result.is_final
    //             );
    //         }
    //     }
    // }

    Ok(())
}
