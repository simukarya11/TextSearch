use std::path::{Path, PathBuf};

#[allow(non_snake_case)]
use crate::OutputHandler::*;

/*----------Struct TextSearch------------------------------------------*/
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct TextSearch {
    searchDirectory: PathBuf,
    textToSearch: String,
    display: Display,
}

#[allow(non_snake_case)]
#[allow(dead_code)]
impl TextSearch {
    pub fn new() -> Self {
        TextSearch {
            searchDirectory: PathBuf::new(),
            textToSearch: String::new(),
            display: Display::new(),
        }
    }

    pub fn setTextToSearch(&mut self, textToSeach: &str) {
        self.textToSearch = textToSeach.to_string();
    }
}

/*----------Impl Struct TextSearch for DirectoryEvent------------------------------------------*/

#[allow(non_snake_case)]
pub trait DirectoryEvent {
    fn setSearchDirectory(&mut self, dir: &Path);
    fn searchFile(&mut self, file: &Path);
}

impl DirectoryEvent for TextSearch {
    fn setSearchDirectory(&mut self, dir: &Path) {
        self.searchDirectory = dir.to_path_buf();
        self.display.setDirectory(dir);
    }

    fn searchFile(&mut self, file: &Path) {
        // TODO - Implement this function
        /* 
            Pretending to search for text in file.
            Function should:
              1. append flnm to dir
              2. attempt to open file
              3. search for text
              4. send result to display 
        */
        if self.textToSearch == "BuildOn" {
            self.display.displaySearchResult((file, true, &self.textToSearch));
        }
        else {
            self.display.displaySearchResult((file, false, &self.textToSearch));
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    fn setup() -> TextSearch {
        TextSearch::new()
    }

    #[test]
    fn testSetDirectory() {
        let mut textSearch = setup();
        textSearch.setSearchDirectory(&Path::new("TEST"));
        assert_eq!(textSearch.searchDirectory, Path::new("TEST").to_path_buf());
    }

    #[test]
    fn testSearchFile() {
        let mut textSearch = setup();
        textSearch.searchFile(&Path::new("TEST"));
    }
}
