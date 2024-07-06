use clap::{Parser, Subcommand};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::{fs, io};
use std::io::Write;
use std::error::Error;
use std::path::PathBuf;

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;
use std::thread;
use std::sync::mpsc;

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

    Clear {
    },

    Exit {
    },
}

#[derive(Debug)]
enum InputEvent {
    Char(char),
    Enter,
    Tab,
    Backspace,
}

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        print!("\ncli_tool> ");
        io::stdout().flush()?;
        let _ = enable_raw_mode();
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            loop {
                if let Ok(Event::Key(KeyEvent { code, modifiers: _ })) = event::read() {
                    match code {
                        KeyCode::Char(c) => {
                            let _ = tx.send(InputEvent::Char(c));
                        }
                        KeyCode::Enter => {
                            let _ = tx.send(InputEvent::Enter);
                            break;
                        }
                        KeyCode::Tab => {
                            let _ = tx.send(InputEvent::Tab);
                            break;
                        }
                        KeyCode::Backspace => {
                            let _ = tx.send(InputEvent::Backspace);
                        }
                        _ => {}
                    }
                }
            }
        });

        let mut input = String::new();
        loop {
            if let Ok(event) = rx.recv_timeout(Duration::from_millis(10)) {
                match event {
                    InputEvent::Char(c) => {
                        input.push(c);
                        print!("{}", c);
                        io::stdout().flush()?;
                    }
                    InputEvent::Enter => {
                        disable_raw_mode()?;
                        print!("\n");
                        cli_match(input.clone())?;
                        break;
                    }
                    InputEvent::Tab => {
                        break;
                    }
                    InputEvent::Backspace => {
                        if !input.is_empty() {
                            input.pop();
                            print!("\x08 \x08");
                            io::stdout().flush()?;
                        }
                    }

                }
            }
        }
    }
}


fn cli_match(input: String) -> Result<(), Box<dyn Error>> {
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

        Commands::Clear {} => {
            clear_terminal();
        },

        Commands::Exit {} => {
            clear_terminal();
            std::process::exit(0);
        }
    }
    return Ok(())
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

fn clear_terminal() {
    print!("{esc}c", esc = 27 as char);
}

fn get_suggestions(input: &str) -> Vec<String> {
    let commands = vec![
        "help".to_string(),
        "exit".to_string(),
        "lsss".to_string(),
        "show".to_string(),
    ];

    commands.into_iter().filter(|cmd| cmd.starts_with(input)).collect()
}

fn handle_events(stdout: &mut io::Stdout) -> crossterm::Result<()> {
    let input = String::new();


    loop {
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Tab => {
                        println!("input is {}", input);
                        let suggestions = get_suggestions(&input);
                        if !suggestions.is_empty() {
                            writeln!(stdout, "\nSuggestions: {:?}", suggestions)?;
                            stdout.flush().unwrap();
                        }
                    }

                    _ => {return Ok(())}
                }
            }
        }
    }
}