use clap::{arg, command, Parser, Subcommand, ValueEnum};
use interface::usb::{get_all_usb_devices, get_exist_usb_devices};
use parse::parse_toml_list;

mod interface;
mod controller;
mod parse;

struct Device {
    name: String,
    func: fn(),
}

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
        device: String,
    },
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
            let usb_device_list = get_all_usb_devices();
            println!("Now detect USB Devices");
            let device_list = parse_toml_list("devicelist.toml");
            let exist_usb_list = get_exist_usb_devices(&device_list);
        }
        SubCommands::Get { device, prop } => {
            println!("device name: {:?}", device)
        }
        SubCommands::Set { device } => {
            println!("device name: {:?}", device)
        }
    }
}
