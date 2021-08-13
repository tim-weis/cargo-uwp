#![cfg(target_os = "windows")]
#![forbid(unsafe_code)]

use structopt::StructOpt;

mod cargo;
mod data;
mod ops;

use ops::New;

#[derive(Debug, StructOpt)]
#[structopt(bin_name = "cargo")]
enum Opt {
    #[structopt(
        name = "uwp",
        about = "Custom cargo command to create, manage, and package UWP applications"
    )]
    Uwp {
        #[structopt(subcommand)]
        subcommand: Subcommand,
    },
}

#[derive(StructOpt, Debug)]
enum Subcommand {
    #[structopt(about = "Creates a new UWP cargo package")]
    New(New),
}

fn main() {
    let Opt::Uwp { subcommand } = Opt::from_args();
    let result = match subcommand {
        Subcommand::New(new) => new.perform(),
    };

    if let Err(ref e) = result {
        println!("{}: {}", console::style("error").red().bright().bold(), e);
    }
}
