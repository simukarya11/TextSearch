#[allow(non_snake_case)]
mod OutputHandler;
mod text_search;

#[allow(unused_imports)]
pub use text_search::*;
#[allow(unused_imports)]
pub use OutputHandler::*;

#[allow(unused_imports)]
use std::path::{Path, PathBuf};

#[allow(non_snake_case)]
pub fn testRunLibrary() {
    print!("\n  -- demonstrate structure for Step#1 --\n");
    let mut finder = TextSearch::new();
    finder.setTextToSearch("BuildOn");
    finder.setSearchDirectory(&Path::new("Dir1"));
    finder.searchFile(&Path::new("file1.txt"));
    finder.setTextToSearch("BuildOff");
    finder.setSearchDirectory(&Path::new("file2.txt"));
    finder.searchFile(&Path::new("Dir2"));
    finder.setTextToSearch("abc");
    finder.searchFile(&Path::new("file3.txt"));
    finder.setTextToSearch("123");
    finder.searchFile(&Path::new("file4.txt"));

    print!("\n\n  That's all Folks!\n\n");
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
