#[macro_use]
extern crate rocket;

mod config;

use rocket::response::content::RawHtml;
use rocket::State;
use tera::{Context, Tera};

struct App {
    pages: Vec<RawHtml<String>>,
    tera: Tera,
    config: Config,
    db_connection: 
}

impl App {}

#[get("/")]
fn index() -> RawHtml<String> {
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
fn rocket() -> _ {
    let app = App {
        pages: todo!(),
        tera: todo!(),
    };

    rocket::build().mount("/", routes![index]).manage(app);
}
