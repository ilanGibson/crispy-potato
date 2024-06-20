use clap::{Parser, Subcommand};
use std::{fs, io};
use std::io::Write;
use std::error::Error;
use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Ls {
        #[arg(short)]
        list: bool,

        #[arg(short)]
        all: bool,

        path: Option<PathBuf>,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        print!("\ncli_tool> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("fail");

        let mut args: Vec<&str> = input.split_whitespace().collect();
        args.insert(0, "cli_tool");

        let cli = Cli::parse_from(&args);


        match &cli.command {
            Commands::Ls { list: _, all, path} => {
                let default_path = PathBuf::from(".");
                if *all {
                    let _ = list_files(path.as_ref().unwrap_or(&default_path).clone(), Some(all));
                } else {
                    let _ = list_files(path.as_ref().unwrap_or(&default_path).clone(), None);
                }
            },
        }
    }
}

fn list_files(path: PathBuf, all: Option<&bool> ) -> Result<(), Box<dyn Error>> {
    let hidden_files = all.unwrap_or(&false);
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.file_name()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    for entry in entries {
        // convert entry to String
        let entry = entry.to_string_lossy().to_string();
        // if entry does not start with "." (aka is not hidden file) print it
        if !entry.starts_with('.') {
            print!("{}\t", entry);
        // if entry starts with "." (aka is hidden file) and show hidden file flag is active, print it
        } else if entry.starts_with('.') && *hidden_files {
            print!("{}\t", entry);
        // if entry starts with "." (aka is hidden file) but show hidden file flag is inactive, do nothing
        } else {
            continue;
        }
    }

    Ok(())
}