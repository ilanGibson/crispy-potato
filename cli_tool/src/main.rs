use clap::{Parser, Subcommand};
use std::path::PathBuf;


#[derive(Parser)]
#[command(version, about, long_about = "test2")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "test")]
    Ls {
        #[arg(short, help = "list all directories including hidden")]
        list: bool,

        file: Option<PathBuf>,
    },

    #[command(about = "exit program")]
    Exit {},
}


use std::{fs, io};
fn main() -> io::Result<()> {

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Ls { list, file  }) => {
            if *list {
                let entries = fs::read_dir(".")?;

                for entry in entries {
                    let entry = entry?;
                    let path = entry.path();
                    let filename = path.file_name().unwrap();
                    println!("{}, {:?}", filename.to_string_lossy(), file);
                }

                Ok(())

            } else {
                Ok(())
            }
        }

        Some(Commands::Exit {}) => {
            println!("test");
            Ok(())
        }

        None => Ok(())
    }
}




// fn main() {
//     let cli = Cli::parse();

//     match &cli.command {
//         Some(Commands::Ls { list }) => {
//             if *list {
//                 println!("Long listing all files in the current directory");
//             } else {
//                 println!("Short listing all files in the current directory");
//             }
//         }
//         None => {}
//     }
// }