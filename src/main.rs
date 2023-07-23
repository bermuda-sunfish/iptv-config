#![warn(missing_docs)]

//! This program is intended to automate the process of finding, verifying, and compiling a list of IPTV channels. 

use log::debug;
use anyhow::Result;
use std::env;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// The main entry point into the application - it reads configuration from the `.env` file
/// if present, initializes the logger, and invokes the `run` function in the command module
/// which parses the command line arguments and dispatches to the appropriate module.
fn main() -> Result<()> {
    init_env();
    env_logger::init();
    debug!("The logger is initialized with the default settings");
    let args = Args::parse();
    debug!("{:?}", args);

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(args.file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if !ip.is_empty() {
                    debug!("Processing URL: {}", ip);
                    let r = search_response(&ip, Some(&args.search));
                    debug!("Result: {:?}", r);
                }
            }
        }
    }

    debug!("Command completed");
    Ok(())
}

/// Initializes the application from the dot-env or the defaults. This is done before
/// logging is initialized, so logging is not supported yet.
fn init_env() {
    if let Err(_) = dotenv::dotenv() {
        // the RUST_LOG environmental variable is not set in command line
        if let Err(_) = env::var("RUST_LOG") {
            env::set_var("RUST_LOG", "info");
        }
    }
}

/// Command line arguments for the program.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The input file with the collection of links to verify
    #[arg(short, long, env="IPTV_SRC_FILE")]
    file: String,

    /// The search term to use on responses
    #[arg(short, long, env="IPTV_SEARCH")]
    search: String,

    /// The output file to write m3u8 stream to
    #[arg(short, long, env="IPTV_TARGET_FILE")]
    output: String,
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn search_response(url: &str, search_term: Option<&str>) -> Result<()> {
    debug!("Retrieving url: {}", url);
    let resp = reqwest::blocking::get(url)?.text()?;
    if let Some(search_term) = search_term {
        debug!("Looking for a search term: {}", search_term);
        for line in resp.lines() {
            if line.contains(search_term) {
                debug!("Found: {}", line);
            }
        }
    } else {
        debug!("Returning the result without searching");
    }
    Ok(())
}