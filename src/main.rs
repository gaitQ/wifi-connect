#![recursion_limit = "1024"]

#[macro_use]
extern crate log;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;

extern crate clap;
extern crate env_logger;
extern crate iron;
extern crate iron_cors;
extern crate mount;
extern crate network_manager;
extern crate nix;
extern crate params;
extern crate persistent;
extern crate router;
extern crate serde_json;
extern crate staticfile;

mod config;
mod dnsmasq;
mod errors;
mod exit;
mod logger;
mod network;
mod privileges;
mod server;

use std::io::Write;
use std::path;
use std::process;
use std::thread;

use config::get_config;
use std::sync::mpsc::channel;
use errors::*;
use exit::block_exit_signals;
use exit::ExitEvent;
use network::{network_init, network_thread};
use privileges::require_root;

fn main() {
    if let Err(ref e) = run() {
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "\x1B[1;31mError: {}\x1B[0m", e).expect(errmsg);

        for inner in e.iter().skip(1) {
            writeln!(stderr, "  caused by: {}", inner).expect(errmsg);
        }

        process::exit(exit_code(e));
    }
}

fn run() -> Result<()> {
    block_exit_signals()?;

    logger::init();

    require_root()?;

    // Channels to signal exit events across threads
    let (exit_tx, exit_rx) = channel();

    // Starts network manger & deletes current AP
    network_init(&get_config())?;

    let config = get_config();

    let network_thread_handle = thread::spawn(move || {
        network_thread(&config, &exit_tx);
    });

    // Blocks unit a thread send an exit event
    match exit_rx.recv() {
        Ok(result) => match result {
            Ok(event) => match event {
                ExitEvent::ExitSignal => info!("Exiting: Signal"),
                ExitEvent::InternetConnected => info!("Exiting: Internet connected"),
                ExitEvent::WiFiConnected => info!("Exiting: WiFi connected"),
                ExitEvent::Timeout => info!("Exiting: Timeout"),
                ExitEvent::UnexpectedExit => info!("Exiting: Unexpectedly"),
            },
            Err(e) => {
                error!("Exiting: Error {}", e.to_string());
                return Err(e.into());
            }
        },
        Err(e) => {
            error!("Exiting: Receive Error {}", e.to_string());
            return Err(e.to_string().into());
        }
    }

    // Join the network thread to ensure it completes gracefully
    let _ = network_thread_handle.join();

    Ok(())
}
