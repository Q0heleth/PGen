#![allow(unused)]
use anyhow::{Context, Result};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use diesel::{RunQueryDsl, SqliteConnection};
use std::io::BufRead;
use structopt::StructOpt;
use models::Password;
use diesel::prelude::*;
use diesel_migrations::embed_migrations;
use pgen::*;
use std::env;
extern  crate rand;
#[macro_use]
extern crate diesel_migrations;
#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    command: Command,
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
        key: Option<String>,
    },
    List,
    Version,
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
                println!("key  value  description");
                result.into_iter().map(|f|println!("{}  {}",f.key,f)).collect::<Vec<_>>();
            }
            Command::Version => {
                println!("PGen is made by csh, Email:317595241@qq.com db file is in CARGO_HOME");
            }
            Command::Delete {key:k} => {
                use schema::password::dsl::*;
                if let Some(k) =k {
                    diesel::delete(password.filter(key.eq(k))).execute(&conn)?;
                }else {
                    diesel::delete(password).execute(&conn)?;
                }
            }
            Command::Get { key:k } => {
               let ret = query_pwd(&conn, &k)?;
                if ret.is_empty() {
                    eprintln!("key {} not exist",k);
                    return Ok(())
                }
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
