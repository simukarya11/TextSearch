use std::path::{Path, PathBuf};


#[allow(non_snake_case)]
use crate::OutputHandler::*;
use std::fs::File;
use std::io::prelude::*;

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
pub trait DirectorySearchEvent {
    fn setSearchDirectory(&mut self, directory: &Path);
    fn searchFile(&mut self, foundFilePath: &Path);
}

impl DirectorySearchEvent for TextSearch {
    fn setSearchDirectory(&mut self, directory: &Path) {
        self.searchDirectory = directory.to_path_buf();
        self.display.setDirectory(directory);
    }

    #[allow(non_snake_case)]
    fn searchFile(&mut self, foundFilePath: &Path) {
        let mut newFilePath = self.searchDirectory.clone();
        newFilePath.push(foundFilePath);

        let openedFile = File::open(&newFilePath);

        if openedFile.is_err() {
            println!("File does not exist in directory {:?}", newFilePath);
            return;
        }

        let mut fileRawData = openedFile.unwrap();
        let mut fileContentInString = String::new();

        match fileRawData.read_to_string(&mut fileContentInString) {
            Err(reason) => println!("Could not convert raw data of to string due to -> {:?}", reason),
            Ok(_) => {},
        }

        let isTextFound = fileContentInString.find(&self.textToSearch);
        self.display
            .displaySearchResult((&newFilePath, isTextFound.is_some(), &self.textToSearch));
    }
}

/*************************************Unit Test************************************** */
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
