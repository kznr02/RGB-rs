use clap::{arg, command, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(version = "1.0")]
struct Cli {
    #[command(subcommand)]
    commands: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Detect,
    Get {
        #[arg(short)]
        device: String,

        prop: ValType,
    },
    Set {
        #[arg(short)]
        device: String
    }
}

#[derive(Debug, ValueEnum, Clone)]
enum ValType {
    Brightness,
    Color,
}

fn main() {
    let args = Cli::parse();

    match args.commands {
        SubCommands::Detect => {
            println!("You are in detect!");
        },
        SubCommands::Get{device, prop} => {
            println!("device name: {:?}", device)
        },
        SubCommands::Set{device} => {
            println!("device name: {:?}", device)
        }
    }
}
