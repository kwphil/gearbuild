use std::{
    env::args, 
    fs::canonicalize, 
    process::exit,
    path::PathBuf,
};

use colored::Colorize;

mod config;
use config::open_config;

fn arg_parse() -> (String, PathBuf) {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        eprintln!(
            "{} {}",
            "ERROR:".bright_red().bold(),
            "No argument provided".bold()
        );

        exit(1);
    }

    let query = &args[1];
    let mut src_path_s = "./";

    if args.len() >= 3 {
        src_path_s = &args[2];
    }

    let src_path = canonicalize(src_path_s).unwrap_or_else(|err| {
        eprintln!(
            "{} {} `{}`\n  {}",
            "ERROR:".bright_red().bold(),
            "Could not open directory".bold(),
            src_path_s,
            err.to_string().bold()
        );
        
        exit(2);
    });

    println!("Building from {}", src_path.display());
    println!("Query: {query}");

    return (query.to_string(), src_path);
}

fn main() {
    // Grab parameters
    let ( query, src_path ) = arg_parse();

    // Find and open the config file
    let config = open_config(&src_path).unwrap_or_else(|err| {
        let mut config_path = src_path;
        config_path.push("gear.toml");

        eprintln!(
            "{} {} `{}`\n  {}",
            "ERROR:".bright_red().bold(),
            "Could not open config file:".bold(),
            config_path.display(),
            err.to_string().bold()
        );

        exit(2);
    });
}
