#[allow(unused_imports)]
use text_search::*;
pub use text_search::*;

#[allow(unused_imports)]
use std::path::{Path, PathBuf};

fn main() {
    let mut finder = TextSearch::new();
    finder.setTextToSearch("Hi");
    finder.setSearchDirectory(&Path::new("./Examples"));
    finder.searchFile(&Path::new("FileUsedToRunTest.txt"));
}
