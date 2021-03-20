#[allow(unused_imports)]
use text_search::*;
pub use text_search::*;
use DirectoryNavigator::DirectorySearchEvent;

#[allow(unused_imports)]
use std::path::{Path, PathBuf};

#[allow(non_snake_case)]
fn main() {
    let mut finder = TextSearch::new();
    let outputImpl = Display::new();
    finder.setOutputObject(Box::new(outputImpl));
    finder.setTextToSearch("Hi");
    finder.setSearchDirectory(&Path::new("./Examples"));
    finder.searchFile(&Path::new("FileUsedToRunTest.txt"));
}
