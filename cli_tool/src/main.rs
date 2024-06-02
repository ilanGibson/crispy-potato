use clap::{Parser, Subcommand};
use std::{fs, io};
use std::io::Write;
use std::error::Error;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    List {
        #[arg(short)]
        list: bool
    },
}

fn main() {
    loop {
        print!("cli_tool> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("fail");
        println!("{}", input);

        let mut args: Vec<&str> = input.split_whitespace().collect();
        args.insert(0, "cli_tool");
        println!("{:?}", args);

        let cli: Result<Cli, Box<dyn Error>> = match Cli::try_parse_from(args) {
            Ok(cli) => Ok(cli),
            Err(e) => Err(Box::new(e)),
        };


        print!("{:?}", cli);

        match &cli.unwrap().command {
            Commands::List { list } => {
                if *list {
                    print!("lidst")
                //     let entries = fs::read_dir(".")?;

                // for entry in entries {
                //     let entry = entry?;
                //     let path = entry.path();
                //     let filename = path.file_name().unwrap();
                //     println!("{}, {:?}", filename.to_string_lossy(), file);
                // }
                } else {
                    print!("nope")
                }
            },
        }
    }
}