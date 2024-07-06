use rocket::{form::Form, fs::TempFile};
use rocket_dyn_templates::{context, Template};

#[macro_use]
extern crate rocket;

const UPLOAD_DIR: &str = "./markdowns/";

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

#[post("/upload", format = "multipart/form-data", data = "<form>")]
async fn upload_markdown(mut form: Form<Upload<'_>>) -> std::io::Result<()> {
    println!("form.file = {:?}", form.file);

    let name = form.file.name().expect("Error, no name in file");
    let file_name = String::from(UPLOAD_DIR) + name + ".md";

    println!("destination = {}", file_name);
    println!("length = {} bytes", form.file.len());

    form.file.persist_to(&file_name).await?;

    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, upload_markdown])
        .attach(Template::fairing())
}
