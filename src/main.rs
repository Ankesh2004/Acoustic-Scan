use std::env;
use std::process;
use::slog::error;

use clap::{Command, Arg};

const SONGS_DIR: &str = "songs";

pub mod command_handlers;
pub mod socket_handlers;
pub mod shazam;
pub mod utils;
pub mod wav;
pub mod models;
pub mod download;
pub mod db;
pub mod api;

fn main() {
    // create "tmp" folder
    if let Err(e) = utils::create_folder("tmp") {
        let logger = utils::get_logger();
        error!(logger, "Failed to create tmp dir. {}", e);
    }

    // Create SONGS_DIR folder
    if let Err(e) = utils::create_folder(SONGS_DIR) {
        let logger = utils::get_logger();
        error!(logger, "Failed to create songs dir. {}", e);
    }

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Expected 'find', 'download', 'erase', 'save', 'serve', or 'api-server' subcommands");
        process::exit(1);
    }

    match args[1].as_str() {
        "find" => {
            if args.len() < 3 {
                println!("Usage: main.rs find <path_to_wav_file>");
                process::exit(1);
            }
            let file_path = &args[2];
            // command_handlers::find(file_path);

            // a runtime to block on the async function
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(command_handlers::find(file_path));
        }
        "download" => {
            if args.len() < 3 {
                println!("Usage: main.rs download <spotify_url>");
                process::exit(1);
            }
            let url = &args[2];
            command_handlers::download(url);
        }

        // TODO: Implement the "serve" subcommand

        // "serve" => {
        //     // Use clap for flag parsing
        //     let serve_cmd = Command::new("serve")
        //         .arg(
        //             Arg::new("proto")
        //                 .long("proto")
        //                 .default_value("http")
        //                 .help("Protocol to use (http or https)"),
        //         )
        //         .arg(
        //             Arg::new("p")
        //                 .short('p')
        //                 .default_value("5000")
        //                 .help("Port to use"),
        //         );
        //     // Parse the remaining arguments (skip the subcommand)
        //     let matches = serve_cmd.get_matches_from(&args[2..]);
        //     let protocol = matches.get_one::<String>("proto").unwrap();
        //     let port = matches.get_one::<String>("p").unwrap();
        //     command_handlers::serve(protocol, port);
        // }
        "erase" => {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(command_handlers::erase(SONGS_DIR));
        }
        "save" => {
            let save_cmd = Command::new("save")
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .help("Save song with or without YouTube ID")
                        .num_args(0),
                )
                .arg(
                    Arg::new("path")
                        .required(true)
                        .help("Path to wav file or directory"),
                );
            let matches = save_cmd.get_matches_from(&args[2..]);
            let force = matches.contains_id("force");
            let file_path = matches.get_one::<String>("path").unwrap();
            command_handlers::save(file_path, force);
        }
        "api-server" => {
            // Default host and port
            let host = args.get(2).map_or("127.0.0.1", |s: &String| s);
            let port = args.get(3).map_or(8080, |s| s.parse().unwrap_or(8080));
            
            println!("Starting API server on http://{}:{}", host, port);
            
            // Create runtime and start the Actix web server
            let rt = tokio::runtime::Runtime::new().unwrap();
            if let Err(e) = rt.block_on(api::start_server(host, port)) {
                eprintln!("Failed to start API server: {}", e);
                process::exit(1);
            }
        }
        _ => {
            println!("Expected 'find', 'download', 'erase', 'save', 'serve', or 'api-server' subcommands");
            process::exit(1);
        }
    }
}