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
mod dnsmasq;
mod errors;
mod exit;
mod logger;
mod network;
mod connectivity;
mod privileges;
mod server;

use std::io::Write;
use std::path;
use std::process;
use std::sync::mpsc::channel;
use std::thread;

use config::get_config;
use errors::*;
use exit::block_exit_signals;
use network::{init_networking, process_network_commands, NetworkCommand};
use connectivity::{check_internet_connectivity, connectivity_thread};
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
        process_network_commands(&config, &exit_tx);
    });

    thread::spawn(move || {
        connectivity_thread(&exit_tx_conn);
    });

    // Starts network manger & deletes current AP
    init_networking(&get_config())?;

    loop {
        // Wait for exit event
        match exit_rx.recv() {
            Ok(result) => match result {
                Ok(command) => match command.unwrap() {
                    NetworkCommand::Activate => error!("Exit due to activate"),
                    NetworkCommand::Timeout => debug!("Exit due to timeout"),
                    NetworkCommand::Exit => {
                        return Ok(());
                    }
                    NetworkCommand::Connect { .. } => {
                        return Ok(());
                    }
                    NetworkCommand::RestartApp => debug!("User restarted app"),
                },
                Err(e) => {
                    return Err(e.into());
                }
            },
            Err(e) => {
                return Err(e.into());
            }
        }

        // TODO Do this in separate thread
        if let Ok(_) = check_internet_connectivity() {
            info!("Internet connected, exiting");
            return Ok(());
        }
    }
}
