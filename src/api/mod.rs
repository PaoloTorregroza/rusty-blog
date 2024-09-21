use rocket::response::Redirect;
use rocket::{form::Form, fs::TempFile};
use rocket_db_pools::Connection;
use std::env;

use crate::controllers;
use crate::db::BlogDatabase;

// AUTH ENDPOINTS

#[derive(FromForm, Debug)]
pub struct SignupForm {
    pub name: String,
    pub email: String,
    pub password: String,
    pub conf_password: String,
}

#[post("/signup", data = "<new_user>")]
pub async fn signup(db: Connection<BlogDatabase>, new_user: Form<SignupForm>) -> Redirect {
    let result = controllers::register_user(db, new_user.into_inner()).await;

    if result {
        Redirect::to("/signin")
    } else {
        Redirect::to("/signup")
    }
}

#[derive(FromForm)]
pub struct Upload<'f> {
    file: TempFile<'f>,
}

#[post("/upload", format = "multipart/form-data", data = "<form>")]
pub async fn upload_markdown(mut form: Form<Upload<'_>>) -> std::io::Result<()> {
    println!("form.file = {:?}", form.file);

    let upload_dir = env::var("UPLOAD_DIR").expect("UPLOAD_DIR not set in .env");
    let name = form.file.name().expect("Error, no name in file");
    let file_name = String::from(upload_dir) + name + ".md";

    println!("destination = {}", file_name);
    println!("length = {} bytes", form.file.len());

    // form.file.persist_to(&file_name).await?;
    form.file.move_copy_to(&file_name).await?;

    Ok(())
}
