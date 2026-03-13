use loony_cognitive_services::realtime::*;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio::{self, fs::File, try_join};

struct MockStore {
    transcript: String,
}

impl MockStore {
    pub fn new() -> Self {
        Self {
            transcript: "".to_owned(),
        }
    }

    pub fn append(&mut self, transcript: String) {
        self.transcript = format!("{} {}", self.transcript, transcript);
    }

    pub fn print(&self) {
        print!("{}", self.transcript)
    }
}

#[tokio::main]
async fn main() {
    let api_key: String = "ZtJSMQrtJ1Y8j1jh3mp7AmRPJK2eEnuE".to_string();
    let (mut rt_session, mut receive_channel) = RealtimeSession::new(api_key, None).unwrap();

    let test_file_path: PathBuf = PathBuf::new()
        .join("home")
        .join("sankar")
        .join("Music")
        .join("sounds")
        .join("speechmatics.wav");

    let file = File::open("/home/sankar/Music/sounds/speechmatics.wav")
        .await
        .unwrap();

    let mut config: SessionConfig = Default::default();
    let audio_config = models::AudioFormat::new(models::audio_format::Type::File);
    config.audio_format = Some(audio_config);

    let mock_store = Arc::new(Mutex::new(MockStore::new()));
    let mock_store_clone = mock_store.clone();

    let message_task = tokio::spawn(async move {
        while let Some(message) = receive_channel.recv().await {
            match message {
                ReadMessage::AddTranscript(mess) => {
                    mock_store_clone
                        .lock()
                        .unwrap()
                        .append(mess.metadata.transcript);
                }
                ReadMessage::EndOfTranscript(_) => return,
                _ => {}
            }
        }
    });

    let run_task = { rt_session.run(config, file) };

    try_join!(
        async move { message_task.await.map_err(anyhow::Error::from) },
        run_task
    )
    .unwrap();

    mock_store.lock().unwrap().print();
}
