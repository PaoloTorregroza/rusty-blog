use db::BlogDatabase;
use dotenvy::dotenv;
use rocket_db_pools::Database;
use rocket_dyn_templates::{context, Template};

mod api;
mod controllers;
mod db;
mod file_manager;
mod views;

#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> Template {
    Template::render(
        "index",
        context! {test_var: "This is a test context variable"},
    )
}

#[launch]
fn rocket() -> _ {
    dotenv().expect(".env not found");

    rocket::build()
        .mount(
            "/",
            routes![
                index,
                views::blog_view,
                views::signup,
                views::signin,
                api::signup,
                api::upload_markdown
            ],
        )
        .attach(BlogDatabase::init())
        .attach(Template::fairing())
}
