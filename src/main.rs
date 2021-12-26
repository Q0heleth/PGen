#![allow(unused)]
use anyhow::{Context, Result};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use diesel::SqliteConnection;
use std::io::BufRead;
use structopt::StructOpt;
use grrs::*;
extern  crate rand;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    command: Command,
    /// The path to the file to read
    #[structopt(parse(from_os_str), default_value = "./pwd.db")]
    path: std::path::PathBuf,
}

#[derive(Debug)]
pub struct Password {
    value: String,
}
#[derive(StructOpt, Debug)]
#[structopt(name = "command")]
enum Command {
    Set {
        key: String,
        #[structopt(parse(from_str=pwd_from_str))]
        pwd: Password,
    },
    Gen {
        key:String,
        #[structopt(default_value = "12")]
        size: usize,
    },
    Get {
        key: String,
    },
    Delete {
        key: String,
    },
    List,
    Info,
    Upload,
    Sync,
}

fn pwd_from_str(src: &str) -> Password {
    Password {
        value: src.to_string(),
    }
}
fn main() -> Result<()> {
    let cli = Cli::from_args();
    let conn = establish_connection();
    cli.command.apply(conn);
    Ok(())
}

impl Command {
    fn apply(self,conn:SqliteConnection) -> Result<()> {
        match self {
            Command::List => {}
            Command::Info => {}
            Command::Sync => {}
            Command::Get { key } => {
               let ret = query_pwd(&conn, &key).expect("keys not exist");
               print!("{}",ret);
            }
            Command::Set { key, pwd } => {
                insert_pwd(&conn, &key, &pwd.value);
            }
            Command::Gen{key,size} => {
               let rand_string:Vec<_> = thread_rng().sample_iter(&Alphanumeric).take(size).collect();
               let pwd = std::str::from_utf8(&rand_string).unwrap();
               insert_pwd(&conn, &key, pwd);
            }
            _ => {}
        }
        Ok(())
    }
}
