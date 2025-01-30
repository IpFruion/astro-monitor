mod system;
mod tui;

use std::{io::BufRead, net::Ipv4Addr};

use anyhow::anyhow;
use clap::{Parser, Subcommand};
use lx200_io::Client;

fn main() -> anyhow::Result<()> {
    let args = Args::try_parse()?;
    match args.action {
        Some(action) => match action {
            Action::Status => todo!(),
            Action::IORaw(settings) => run_io_raw(settings)?,
        },
        None => tui::start()?,
    }
    Ok(())
}

/// A tool to monitor the telescope and interact with it to do different actions
#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    action: Option<Action>,
}

#[derive(Subcommand)]
enum Action {
    Status,
    IORaw(IORawSettings),
}

#[derive(clap::Args)]
struct IORawSettings {
    addr: Option<Ipv4Addr>,
    port: Option<String>,
}

fn run_io_raw(settings: IORawSettings) -> anyhow::Result<()> {
    let mut client = match (settings.addr, settings.port) {
        (None, None) => return Err(anyhow!("Needs an address or port")),
        (None, Some(port)) => Client::open(&port)?.into_boxed(),
        (Some(addr), None) => Client::connect(addr)?.into_boxed(),
        (Some(addr), Some(_)) => Client::connect(addr)?.into_boxed(),
    };
    let stdin = std::io::stdin();
    let mut buf = [0; 1024];
    println!("Input:");
    for line in stdin.lock().lines() {
        let line = line?;
        client.get_mut().write_all(line.as_bytes())?;
        let bytes = client.get_mut().read(&mut buf)?;
        if bytes == 0 {
            println!("Received Nothing");
            continue;
        }
        let resp = String::from_utf8_lossy(&buf[..bytes]);
        println!("Response: {resp}");
        println!("Input:");
    }
    Ok(())
}
