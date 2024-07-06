use std::env;

use dotenvy::dotenv;
use file_manager::file_exists;
use rocket::{
    form::Form,
    fs::TempFile,
    http::{RawStr, Status},
    tokio::fs,
};
use rocket_dyn_templates::{context, Template};

mod file_manager;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {test_var: "This is a test context variable"},
    )
}

#[derive(FromForm)]
struct Upload<'f> {
    file: TempFile<'f>,
}

#[get("/post/<name>")]
async fn blog_view(name: String) -> Template {
    let upload_dir = env::var("UPLOAD_DIR").expect("UPLOAD_DIR not set in .env");
    let file_dir = String::from(upload_dir) + name.as_str() + ".md";

    if !file_exists(&file_dir) {
        return Template::render("404", context! {});
    }

    let content = fs::read_to_string(&file_dir)
        .await
        .expect("Error reading file");

    Template::render(
        "blog_view",
        context! {
            raw_markdown: content,
        },
    )
}

#[post("/upload", format = "multipart/form-data", data = "<form>")]
async fn upload_markdown(mut form: Form<Upload<'_>>) -> std::io::Result<()> {
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

#[launch]
fn rocket() -> _ {
    dotenv().expect(".env not found");
    rocket::build()
        .mount("/", routes![index, upload_markdown, blog_view])
        .attach(Template::fairing())
}
