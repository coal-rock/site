#[macro_use]
extern crate rocket;

mod cli;
mod config;
mod database;
mod page;
mod template;

use std::{collections::HashMap, path::PathBuf};

use config::Config;
use database::Database;
use page::Page;
use rocket::fs::NamedFile;
use rocket::response::content::RawHtml;
use rocket::State;
use tera::{Context, Tera};

struct App {
    compiled_pages: HashMap<String, RawHtml<String>>,
    pages: Vec<Page>,
    tera: Tera,
    config: Config,
    database: Database,
}

impl App {}

#[get("/")]
async fn index(state: &State<App>) -> RawHtml<String> {
    RawHtml(state.tera.render("index.html", &Context::new()).unwrap())
}

#[get("/static/<category>/<filename>")]
async fn cms(state: &State<App>, category: &str, filename: &str) -> Option<NamedFile> {
    if !state.config.static_routes.contains(&category.to_string()) {
        return None;
    }

    match NamedFile::open(format!(
        "{}/{}/{}",
        state.config.static_dir, category, filename
    ))
    .await
    {
        Ok(file) => Some(file),
        Err(_) => None,
    }
}

#[get("/<path..>")]
async fn pages(state: &State<App>, path: PathBuf) -> Option<NamedFile> {
    None
}

#[launch]
async fn rocket() -> _ {
    let config = Config::default();

    let tera = match Tera::new(&format!("{}**/*.html", &config.template_dir)) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let database = Database::new(&config).await;

    let app = App {
        compiled_pages: HashMap::new(),
        tera,
        database,
        config,
        pages: vec![],
    };

    rocket::build()
        .mount("/", routes![index, cms, pages])
        .manage(app)
}
