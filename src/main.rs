#![recursion_limit = "1024"]

#[macro_use]
extern crate log;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;

#[macro_use]
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
mod connectivity;
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
use std::sync::mpsc::channel;
use std::thread;

use config::get_config;
use connectivity::{check_internet_connectivity, connectivity_thread};
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

    let config = get_config();

    require_root()?;

    if let Ok(_) = check_internet_connectivity() {
        info!("Internet connected, skipping wifi-connect");
        return Ok(());
    }

    // Channels to signal exit events from other threads
    let (exit_tx, exit_rx) = channel();
    let exit_tx_conn = exit_tx.clone();

    thread::spawn(move || {
        network_thread(&config, &exit_tx);
    });

    thread::spawn(move || {
        connectivity_thread(&exit_tx_conn);
    });

    // Starts network manger & deletes current AP
    network_init(&get_config())?;

    // Blocks unit a thread send an exit event
    match exit_rx.recv() {
        Ok(result) => match result {
            Ok(event) => match event {
                ExitEvent::ExitSignal => {
                    info!("Exiting: Signal");
                    return Ok(());
                }
                ExitEvent::InternetConnected => {
                    info!("Exiting: Internet connected");
                    return Ok(());
                }
                ExitEvent::WiFiConnected => {
                    info!("Exiting: WiFi connected");
                    return Ok(());
                }
                ExitEvent::Timeout => {
                    info!("Exiting: Timeout");
                    return Ok(());
                }
            },
            Err(e) => {
                error!("Exiting: Error {}", e.to_string());
                return Err(e.into());
            }
        },
        Err(e) => {
            error!("Exiting: Receive Error {}", e.to_string());
            return Err(e.into());
        }
    }
}
