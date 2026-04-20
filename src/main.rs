use std::{
    env::args, 
    fs::{
        canonicalize,
        create_dir 
    },
    process::exit,
    path::PathBuf,
};

use colored::Colorize;

mod config;
mod init;
use config::open_config;
use init::init_command;

fn arg_parse() -> (String, PathBuf, usize) {
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

    if query == "new" {
        if args.len() < 3 {
            eprintln!(
                "{} {}",
                "ERROR:".bright_red().bold(),
                "Expected the name of a directory".bold()
            );

            exit(1);
        }

        create_dir(src_path_s).unwrap_or_else(|err| {
            eprintln!(
                "{} {} `{}`\n  {}",
                "ERROR:".bright_red().bold(),
                "Could not create directory:".bold(),
                src_path_s,
                err.to_string().bold()
            );

            exit(1);
        });

        init_command(src_path_s.into());
    }

    let src_path = canonicalize(src_path_s).unwrap_or_else(|err| {
        eprintln!(
            "{} {} `{}`\n  {}",
            "ERROR:".bright_red().bold(),
            "Could not open directory".bold(),
            src_path_s,
            err.to_string().bold()
        );
        
        exit(1);
    });

    return (query.to_string(), src_path, args.len());
}

fn main() {
    // Grab parameters
    let ( query, src_path, _nargs ) = arg_parse();

    if query == "new" || query == "init" {
        init_command(src_path);
    }

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

        exit(1);
    });
}
