#[macro_use]
extern crate rocket;

mod cli;
mod config;
mod database;

use config::Config;
use database::Database;
use rocket::response::content::RawHtml;
use rocket::State;
use tera::{Context, Tera};

struct App {
    pages: Vec<RawHtml<String>>,
    tera: Tera,
    config: Config,
    database: Database,
}

impl App {}

#[get("/")]
async fn index() -> RawHtml<String> {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    RawHtml(tera.render("index.html", &Context::new()).unwrap())
}

#[launch]
async fn rocket() -> _ {
    let app = App {
        pages: todo!(),
        tera: todo!(),
        config: todo!(),
        database: todo!(),
    };

    rocket::build().mount("/", routes![index]).manage(app);
}
