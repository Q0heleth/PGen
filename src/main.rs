#![allow(unused)]
use anyhow::{Context, Result};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use diesel::SqliteConnection;
use std::io::BufRead;
use structopt::StructOpt;
use models::Password;
use diesel_migrations::embed_migrations;
use grrs::*;
extern  crate rand;
#[macro_use]
extern crate diesel_migrations;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    command: Command,
    /// The path to the file to read
    #[structopt(parse(from_os_str), default_value = "./pwd.db")]
    path: std::path::PathBuf,
}

// #[derive(Debug)]
// pub struct Password {
//     value: String,
// }
#[derive(StructOpt, Debug)]
#[structopt(name = "command")]
enum Command {
    Set {
        key: String,
        pwd: String,
        #[structopt(short = "m")]
        desc:Option<String>,
    },
    Gen {
        key:String,
        #[structopt(default_value = "12",short = "s")]
        size: usize,
        #[structopt(short = "m")]
        desc:Option<String>,
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

// fn pwd_from_str(src: &str) -> Password {
//     Password {
//         value: src.to_string(),
//
//     }
// }
embed_migrations!();
fn main() -> Result<()> {
    let cli = Cli::from_args();
    let conn = establish_connection();
    embedded_migrations::run(&conn);
    cli.command.apply(conn);
    Ok(())
}

impl Command {
    fn apply(self,conn:SqliteConnection) -> Result<()> {
        match self {
            Command::List => {
                let result = list_pwd(&conn)?;
                result.into_iter().map(|f|println!("{}",f)).collect::<Vec<_>>();
            }
            Command::Info => {}
            Command::Sync => {}
            Command::Get { key } => {
               let ret = query_pwd(&conn, &key).expect("keys not exist");
               ret.into_iter().map(|f|println!("{}",f)).collect::<Vec<_>>();
            }
            Command::Set { key, pwd ,desc} => {
                insert_pwd(&conn, &key, &pwd,desc.as_deref());
            }
            Command::Gen{key,size,desc} => {
               let rand_string:Vec<_> = thread_rng().sample_iter(&Alphanumeric).take(size).collect();
               let pwd = std::str::from_utf8(&rand_string).unwrap();
               insert_pwd(&conn, &key, pwd,desc.as_deref());
                println!("{}",pwd);
            }
            _ => {}
        }
        Ok(())
    }
}
