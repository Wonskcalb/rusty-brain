#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

mod types;

use rocket::request::Form;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

use types::PasteData;
use types::PasteID;

#[get("/")]
fn index() -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();

    // TODO!Replace the base_paste_url by some URL env or something
    context.insert("base_paste_url", "http://localhost:8000/brain");
    context.insert("paste_id", PasteID::new().val());
    Template::render("home", &context)
}

#[post("/", data = "<paste_form>")]
fn new(paste_form: Form<PasteData>) -> String {
    let paste = paste_form.into_inner();

    // TODO: The Paste being valid, now
    //  5. Redirect the user to {{ base_paste_url }}/{{ paste.seed_id }}

    paste.save().expect("Could not write file");

    String::new()
}

// #[get("/pastes/<paste_id>")]
// fn paste(paste_id: String) -> Template {
//     PasteData::find(paste_id).expect("Oops!");
//
//     let context: HashMap<&str, &str> = HashMap::new();
//     Template::render("paste", &context)
// }

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index, new])
        .mount("/assets", StaticFiles::from("src/assets"))
        .launch();
}
