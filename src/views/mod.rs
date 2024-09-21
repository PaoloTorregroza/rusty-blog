use std::env;

use rocket::tokio::fs;
use rocket_dyn_templates::{context, Template};

use crate::file_manager;

// Principal view responsible for rendering each uploadedd blog post
#[get("/post/<name>")]
pub async fn blog_view(name: String) -> Template {
    let upload_dir = env::var("UPLOAD_DIR").expect("UPLOAD_DIR not set in .env");
    let file_dir = String::from(upload_dir) + name.as_str() + ".md";

    println!("{}", file_dir);
    if !file_manager::file_exists(&file_dir) {
        return Template::render("404", context! {});
    }

    let content = fs::read_to_string(&file_dir)
        .await
        .expect("Error reading file")
        .replace("\n", "\\n");

    println!("{}", content);

    Template::render(
        "blog_view",
        context! {
            raw_markdown: content,
        },
    )
}

// AUTH VIEWS
#[get("/signup")]
pub fn signup() -> Template {
    Template::render("auth/signup", context! {})
}

#[get("/signin")]
pub fn signin() -> Template {
    Template::render("auth/signin", context! {})
}
