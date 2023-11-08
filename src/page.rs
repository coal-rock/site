use std::path::PathBuf;

use crate::template::Template;

pub struct Page {
    title: String,
    description: Option<String>,
    content: String,
    date: String,
    serve_path: PathBuf,
    template: String,
}

impl Page {}
