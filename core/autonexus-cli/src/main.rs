use autonexus_common::CanFrame;
use autonexus_hw::{HardwareAdapter, MockAdapter};
use autonexus_protocols::UdsSession;
use clap::{Parser, Subcommand};
use colored::*;
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "AutoNexus")]
#[command(about = "Advanced Automotive Communication CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List available hardware adapters
    List,
    /// Send a CAN frame
    SendCan {
        #[arg(short, long)]
        id: u32,
        #[arg(short, long)]
        data: String,
    },
    /// Run UDS command
    Uds {
        #[command(subcommand)]
        sub: UdsCommands,
    },
}

fn parse_hex_u8(s: &str) -> Result<u8, String> {
    u8::from_str_radix(s.trim_start_matches("0x"), 16).map_err(|e| e.to_string())
}

fn parse_hex_u16(s: &str) -> Result<u16, String> {
    u16::from_str_radix(s.trim_start_matches("0x"), 16).map_err(|e| e.to_string())
}

#[derive(Subcommand)]
enum UdsCommands {
    /// Request diagnostic session control
    Session {
        #[arg(short, long, value_parser = parse_hex_u8)]
        session: u8,
    },
    /// Read data by identifier
    Read {
        #[arg(short, long, value_parser = parse_hex_u16)]
        did: u16,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // For now, we use the MockAdapter
    let adapter = Arc::new(MockAdapter::new());
    adapter.open().await?;

    match cli.command {
        Commands::List => {
            println!("{} Available Adapters:", "AutoNexus".blue().bold());
            println!("- {}: {}", adapter.name().green(), adapter.description());
        }
        Commands::SendCan { id, data } => {
            let data_bytes: Vec<u8> = data
                .split(',')
                .filter_map(|s| u8::from_str_radix(s.trim_start_matches("0x").trim(), 16).ok())
                .collect();
            let frame = CanFrame {
                id,
                data: data_bytes,
                is_extended: false,
                is_fd: false,
                timestamp: 0,
            };
            adapter.send_can(frame).await?;
            println!(
                "{} CAN frame sent to ID: {:#X}",
                "SUCCESS".green().bold(),
                id
            );
        }
        Commands::Uds { sub } => {
            let uds = UdsSession::new(adapter.clone(), 0x7E0, 0x7E8);
            match sub {
                UdsCommands::Session { session } => {
                    let response = uds.diagnostic_session_control(session).await?;
                    println!(
                        "{} UDS Session Control Response: {:X?}",
                        "SUCCESS".green().bold(),
                        response
                    );
                }
                UdsCommands::Read { did } => {
                    let response = uds.read_data_by_identifier(did).await?;
                    println!(
                        "{} UDS Read DID {:#X} Response: {:X?}",
                        "SUCCESS".green().bold(),
                        did,
                        response
                    );
                }
            }
        }
    }

    Ok(())
}
