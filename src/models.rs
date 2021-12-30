#[derive(Queryable)]
pub struct Password{
    pub id:Option<i32>,
    pub key:String,
    pub value:String,
    pub description:Option<String>
}

use std::fmt::{Display, Formatter};
use super::schema::password;
#[derive(Insertable)]
#[table_name="password"]
pub struct NewPassword<'a> {
    pub key:&'a str,
    pub value:&'a str,
    pub description:Option<&'a str>
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(ref desc) = self.description {
            write!(f,"{}    {}",self.value,desc)
        }else {
            write!(f,"{}",self.value)
        }
    }
}
// #[derive(Queryable)]
// pub struct Post {
//     pub id: Option<i32>,
//     pub title: String,
//     pub body: String,
//     pub published: bool,
// }
//
//
// use super::schema::posts;
//
// #[derive(Insertable)]
// #[table_name="posts"]
// pub struct NewPost<'a> {
//     pub title: &'a str,
//     pub body: &'a str,
// }