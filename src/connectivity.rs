use crossbeam::channel::Sender;
use std::thread;
use std::time::Duration;
use errors::*;
use exit::{ExitEvent, ExitResult};

pub fn check_internet_connectivity() -> Result<()> {
    let url = "https://www.google.com";
    let response = reqwest::blocking::get(url);

    match response {
        Ok(response) => {
            if response.status().is_success() {
                Ok(())
            } else {
                Err("No internet connection.".into())
            }
        }
        Err(_) => {
            Err("Failed to send get request.".into())
        }
    }
}

pub fn connectivity_thread(exit_tx: &Sender<ExitResult>) {
    let exit_tx = exit_tx.clone();

    loop {
        if let Ok(_) = check_internet_connectivity() {
            info!("Internet connected.");
            let _ = exit_tx.send(Ok(ExitEvent::InternetConnected));
            return;
        }

        thread::sleep(Duration::from_secs(10));
    }
}
