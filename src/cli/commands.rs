use crate::cli::{
    actions::{add_password, generate_password, list_passwords, remove_password, show_password},
    args::{Args, Length},
};
use clap::Parser;

use super::actions::get_random_password;

#[derive(Parser, Debug)]
pub enum Command {
    Add {
        #[clap(short, long)]
        service: String,
        #[clap(short, long, aliases=&["user"])]
        username: Option<String>,
        #[clap(short, long)]
        password: Option<String>,
        #[clap(short, long)]
        master: Option<String>,
        #[clap(short, long, default_value_t = false)]
        generate: bool,
        #[clap(short, long, default_value_t = Length::Sixteen)]
        length: Length,
        #[clap(long, default_value_t = false)]
        symbols: bool,
        #[clap(long, default_value_t = true)]
        uppercase: bool,
        #[clap(long, default_value_t = true)]
        lowercase: bool,
        #[clap(long, default_value_t = true)]
        numbers: bool,
    },
    #[clap(
        about = "Generate a password with the specified properties [default: length=16, symbols=false, uppercase=true, lowercase=true, numbers=true, count=1]",
        long_about = "Generate a password with the specified properties [default: length=16, symbols=false, uppercase=true, lowercase=true, numbers=true, count=1]"
    )]
    Generate {
        #[clap(short, long, default_value_t = Length::Sixteen)]
        length: Length,
        #[clap(short, long, default_value_t = false)]
        symbols: bool,
        #[clap(short('U'), long, default_value_t = true)]
        uppercase: bool,
        #[clap(short('u'), long, default_value_t = true)]
        lowercase: bool,
        #[clap(short, long, default_value_t = true)]
        numbers: bool,
        #[clap(short, long, default_value_t = 1)]
        count: usize,
    },
    List {
        #[clap(short, long)]
        master: Option<String>,
        #[clap(short, long, default_value_t = false, aliases=&["show", "show-passwords", "reveal"])]
        show_passwords: bool,
    },
    Remove {
        #[clap(short, long)]
        service: String,
        #[clap(short, long, aliases=&["user"])]
        username: Option<String>,
        #[clap(short, long)]
        master: Option<String>,
    },
    Show {
        #[clap(short, long)]
        service: String,
        #[clap(short, long, aliases=&["user"])]
        username: Option<String>,
        #[clap(short, long)]
        master: Option<String>,
    },
}

impl Command {
    pub fn map(cli_args: Args) {
        match cli_args.command {
            Command::Add {
                service,
                username,
                password,
                master,
                generate,
                length,
                symbols,
                uppercase,
                lowercase,
                numbers,
            } => {
                let password = if generate {
                    Some(get_random_password(
                        length, symbols, uppercase, lowercase, numbers,
                    ))
                } else {
                    password
                };
                add_password(service, username, master, password)
            }
            .expect("Failed to add password"),
            Command::Generate {
                length,
                symbols,
                uppercase,
                lowercase,
                numbers,
                count,
            } => generate_password(length, symbols, uppercase, lowercase, numbers, count),
            Command::List { master, show_passwords } => list_passwords(master, show_passwords).expect("Failed to get passwords"),
            Command::Remove {
                service,
                username,
                master,
            } => remove_password(service, username, master).expect("Failed to remove password"),
            Command::Show {
                service,
                username,
                master,
            } => show_password(service, username, master).expect("Failed to get passwords"),
        }
    }
}
