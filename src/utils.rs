use include_dir::{include_dir, Dir};
use std::path::Path;

// In the future, this should probably get worked on as a LocalResource
static CONTENT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/content");

pub fn get_content(file: &str) -> &str {
    CONTENT_DIR.get_file(file).unwrap().contents_utf8().unwrap()
}
