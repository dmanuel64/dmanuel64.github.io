use comrak::{parse_document, Arena, Options};
use include_dir::{include_dir, Dir};
use leptos::logging;
use std::path::{Path, PathBuf};

use crate::errors::ContentError;

// In the future, this should probably get worked on as a LocalResource
static CONTENT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/content");

#[derive(PartialEq, Eq)]
pub enum ContentType {
    Blog,
    Project,
}

pub fn get_content(content_type: ContentType, name: &str) -> Result<&'static str, ContentError> {
    let path: PathBuf = format!(
        "{}/{name}",
        if content_type == ContentType::Blog {
            "blogs"
        } else {
            "projects"
        }
    )
    .into();
    let path = path.with_extension("md");
    CONTENT_DIR
        .get_file(path)
        .ok_or_else(|| ContentError::ContentNotFound(name.into()))?
        .contents_utf8()
        .ok_or(ContentError::UTF8Error)
}

pub fn get_project(name: &str) -> Result<&'static str, ContentError> {
    let arena = Arena::new();
    let content = get_content(ContentType::Project, name);
    let mut options = Options::default();
    options.extension.front_matter_delimiter = Some("---".to_owned());
    let ast = parse_document(&arena, content.as_ref().unwrap(), &options);
    logging::log!("{:?}", ast);
    content
    
}

pub fn get_blog(name: &str) -> Result<&'static str, ContentError> {
    get_content(ContentType::Blog, name)
}
