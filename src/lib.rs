#[macro_use]
extern crate diesel;
use anyhow::Result;
pub mod schema;
pub mod models;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use models::Password;
//use models::Password;
use std::env;
pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("CARGO_HOME")
        .expect("DATABASE_URL must be set") + "/pgen.db";
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use crate::models::NewPassword;

//use self::models::{Post, NewPost};

// pub fn create_post<'a>(conn: &SqliteConnection, title: &'a str, body: &'a str) {
//     use schema::posts;
//
//     let new_post = NewPost {
//         title: title,
//         body: body,
//     };
//
//
//     diesel::insert_into(posts::table)
//         .values(&new_post)
//         .execute(conn);
// }

pub fn insert_pwd(conn:&SqliteConnection,key:&str,value:&str,description:Option<&str>){
    use schema::password;
    let new_pwd = NewPassword {
        key,
        value,
        description
    };
    diesel::insert_into(password::table)
        .values(&new_pwd)
        .execute(conn)
        .unwrap();
}

pub fn query_pwd(conn:&SqliteConnection,k:&str) -> Result<Vec<Password>, diesel::result::Error> {
    use schema::password::dsl::*;
   password.filter(key.eq(k)).load::<Password>(conn)
}

pub fn list_pwd(conn:&SqliteConnection) -> QueryResult<Vec<Password>> {
    use schema::password::dsl::*;
    password.load::<Password>(conn)
}