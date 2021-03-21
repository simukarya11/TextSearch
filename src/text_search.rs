#![allow(non_snake_case)]
#![allow(dead_code)]

use std::path::{Path, PathBuf};

use crate::OutputHandler::*;
use std::fs::File;
use std::io::prelude::*;
use DirectoryNavigator::DirectorySearchEventTrait;

/*----------Struct TextSearch------------------------------------------*/
pub struct TextSearch {
    searchDirectory: PathBuf,
    textToSearch: String,
    display: Option<Box<dyn OutputTrait>>,
}

impl TextSearch {
    pub fn new() -> Self {
        TextSearch {
            searchDirectory: PathBuf::new(),
            textToSearch: String::new(),
            display: None,
        }
    }

    pub fn setTextToSearch(&mut self, textToSearch: &str) {
        self.textToSearch = textToSearch.to_string();
    }

    pub fn setOutputObject(&mut self, displayImpl: Box<dyn OutputTrait>) {
        self.display.replace(displayImpl);
    }
}

/*----------Impl Struct TextSearch for DirectoryEvent Trait initialized by DirectoryNavigator------------------------------------------*/

impl DirectorySearchEventTrait for TextSearch {
    fn setSearchDirectory(&mut self, directory: &Path) {
        self.searchDirectory = directory.to_path_buf();
        if let Some(plug) = &mut self.display {
            plug.setDirectory(directory);
        }
    }

    fn searchFile(&mut self, foundFilePath: &Path) {
        let mut newFilePath = self.searchDirectory.clone();
        newFilePath.push(foundFilePath);

        let openedFile = self.openFile(&newFilePath);
        if openedFile.is_none() {
            return;
        }

        let fileContentInString = self.readFileToString(&mut openedFile.unwrap());
        if fileContentInString.is_none() {
            return;
        }

        let isTextFound = fileContentInString.unwrap().find(&self.textToSearch);
        self.displayResult(&newFilePath, isTextFound.is_some());
    }
}

impl TextSearch {
    fn openFile(&self, filePath: &std::path::PathBuf) -> Option<File> {
        let openedFile = File::open(&filePath);
        if openedFile.is_err() {
            println!("File does not exist in directory {:?}", filePath);
            return None;
        }
        return Some(openedFile.unwrap());
    }

    fn readFileToString(&self, fileRawData: &mut File) -> Option<String> {
        let mut fileContentInString = String::new();
        match fileRawData.read_to_string(&mut fileContentInString) {
            Err(reason) => {
                println!(
                    "Could not convert raw data of to string due to -> {:?}",
                    reason
                );
                return None;
            }
            Ok(_) => return Some(fileContentInString)
        }
    }

    fn displayResult(&mut self, filePath: &std::path::PathBuf, isTextFound: bool) {
        if let Some(display) = &mut self.display {
            display.displaySearchResult((&filePath, isTextFound, &self.textToSearch));
        }
    }
}

/*
    TODO: Add unit tests for entire File
 */
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
