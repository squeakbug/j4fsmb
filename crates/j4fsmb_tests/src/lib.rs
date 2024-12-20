use std::time::Duration;

use tokio::process::{Command, Child};

use j4fsmb_client::{client::ClientBuilder, Client};

pub struct ServerProcess {
    pub child: Child,
}

impl Default for ServerProcess {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerProcess {
    pub fn new() -> Self {
        let _build_proc = std::process::Command::new("cargo")
            .arg("build")
            .arg("--package")
            .arg("j4fsmb_server")
            .arg("--bin")
            .arg("j4fsmb_server")
            .status()
            .expect("process failed to execute");
        
        let child = Command::new("cargo")
            .arg("run")
            .arg("--package")
            .arg("j4fsmb_server")
            .arg("--bin")
            .arg("j4fsmb_server")
            .kill_on_drop(true)
            .spawn()
            .expect("Failed to start process");

        ServerProcess {
            child,
        }
    }

    pub async fn waited_until_running(self) -> Self {
        let client = ClientBuilder::new();
        tryhard::retry_fn(|| async {
            Ok(())
        })
        .retries(10)
        .exponential_backoff(Duration::from_millis(100))
        .max_delay(Duration::from_secs(10))
        .await
        .expect("Failed to run server");
        self
    }
}