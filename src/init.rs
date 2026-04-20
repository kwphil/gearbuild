use std::{
    path::PathBuf,
    fs::{write, create_dir},
    process::exit
};

use colored::Colorize;

const EXAMPLE_PROG: &str = "
extern(\"C\") fn printf(s: ^char);

printf(\"Hello, World!\");
";

const EXAMPLE_CONFIG: &str = "
[general]

name = \"hello-world\"
version = \"0.1.0\"
";

pub fn init_command(src_path: PathBuf) -> ! {
    if src_path.join("gear.toml").exists() {
        eprintln!(
            "{} {}",
            "ERROR:".bright_red().bold(),
            "Detected an already existing project"
        );
    }

    let code_path = src_path.join("src");

    if create_dir(&code_path).is_err() {
        eprintln!(
            "{} {}\n  {} {}",
            "WARNING:".bright_yellow().bold(),
            "src/ directory failed to create".bold(),
            "HINT:".bright_blue().bold(),
            "You may need to configure gearbuild to point at the correct directory"
        );
    } else {
        write(code_path.join("main.gr"), EXAMPLE_PROG).unwrap();
    }

    write(src_path.join("gear.toml"), EXAMPLE_CONFIG).unwrap();

    exit(0);
}