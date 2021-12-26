#[derive(Queryable)]
pub struct Password{
    pub id:Option<i32>,
    pub key:String,
    pub value:String
}
use super::schema::password;
#[derive(Insertable)]
#[table_name="password"]
pub struct NewPassword<'a> {
    pub key:&'a str,
    pub value:&'a str,
}
#[derive(Queryable)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
    pub published: bool,
}


use super::schema::posts;

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}