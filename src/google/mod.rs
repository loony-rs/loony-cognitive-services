use futures::pin_mut;
use google_cognitive_apis::api::grpc::google::cloud::speechtotext::v2::{
    ExplicitDecodingConfig, RecognitionConfig, StreamingRecognitionConfig,
    StreamingRecognitionFeatures, StreamingRecognizeRequest,
    explicit_decoding_config::AudioEncoding, recognition_config::DecodingConfig,
    streaming_recognize_request::StreamingRequest,
};
use google_cognitive_apis::speechtotext::recognizer_v2::Recognizer;
use log::*;
use std::fs::File;
use std::time::{Duration, Instant};
use tokio::sync::mpsc::Sender;
use tokio_stream::StreamExt;

#[derive(Clone)]
pub struct GoogleSTT {
    pub audio_sink: Option<Sender<StreamingRecognizeRequest>>,
    pub recognizer_id: String,
    pub streaming_recognition_config: StreamingRecognitionConfig,
}

impl GoogleSTT {
    pub async fn new(config: (String, String, i32, i32)) -> Result<Self, ()> {
        let streaming_recognition_config =
            GoogleSTT::get_streaming_recognition_config(config).await?;

        GoogleSTT::new_with_streaming_recognition_config(streaming_recognition_config).await
    }

    pub async fn new_with_streaming_recognition_config(
        streaming_recognition_config: StreamingRecognitionConfig,
    ) -> Result<Self, ()> {
        let project_id = "project-1da9264c-b91c-40ff-993";
        let recognizer_location = format!(
            "projects/{}/locations/global/recognizers/{}",
            project_id, "loony-streaming-1"
        );
        let token = get_access_token().await;
        let bearer_token = format!("Bearer {}", token);

        let mut recognizer = Recognizer::create_streaming_recognizer_from_token(
            bearer_token,
            streaming_recognition_config.clone(),
            None,
            recognizer_location.clone(),
        )
        .await
        .unwrap();

        let audio_sink = recognizer.get_audio_sink();

        tokio::spawn(GoogleSTT::stt_streaming_loop(recognizer));

        Ok(GoogleSTT {
            audio_sink,
            recognizer_id: recognizer_location,
            streaming_recognition_config,
        })
    }

    async fn get_streaming_recognition_config(
        config: (String, String, i32, i32),
    ) -> Result<StreamingRecognitionConfig, ()> {
        let config = StreamingRecognitionConfig {
            config: Some(RecognitionConfig {
                model: config.0,
                language_codes: vec![config.1],
                features: None,
                adaptation: None,
                transcript_normalization: None,
                translation_config: None,
                denoiser_config: None,
                decoding_config: Some(DecodingConfig::ExplicitDecodingConfig(
                    ExplicitDecodingConfig {
                        encoding: AudioEncoding::Linear16 as i32,
                        sample_rate_hertz: config.2,
                        audio_channel_count: config.3,
                    },
                )),
                ..Default::default()
            }),
            config_mask: None,
            streaming_features: Some(StreamingRecognitionFeatures {
                enable_voice_activity_events: false,
                interim_results: true,
                voice_activity_timeout: None,
                endpointing_sensitivity: 0,
            }),
        };
        Ok(config)
    }

    async fn stt_streaming_loop(mut recognizer: Recognizer) {
        let stream = recognizer.streaming_recognize_async_stream().await;
        pin_mut!(stream); // needed for iteration

        while let Some(streaming_recog_result) = stream.next().await {
            println!("streaming_recog_result {:?}", streaming_recog_result);
            println!("reached on streaming_recog_result");

            match streaming_recog_result {
                Err(streaming_recog_err) => {
                    println!("Error: {:?}", streaming_recog_err);
                }
                Ok(streaming_recog_resp) => {
                    println!("streaming_recog_resp: Ok");
                    let mut is_final = false;
                    #[allow(clippy::bind_instead_of_map)]
                    let transcript_text = streaming_recog_resp
                        .results
                        .first()
                        .and_then(|recognition_result| {
                            is_final = recognition_result.is_final;
                            recognition_result.alternatives.first()
                        })
                        .and_then(|alternative| Some(alternative.transcript.to_owned()));
                    println!("{:?}", transcript_text);
                }
            }
        }
    }
}

async fn get_access_token() -> String {
    let output = std::process::Command::new("gcloud")
        .args(["auth", "application-default", "print-access-token"])
        .output()
        .expect("failed to get token");

    String::from_utf8(output.stdout).unwrap().trim().to_string()
}
