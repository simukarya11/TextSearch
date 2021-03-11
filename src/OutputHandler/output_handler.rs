use std::path::{Path, PathBuf};

/*----------Struct Display------------------------------------------*/

#[allow(dead_code)]
pub struct Display {
    searchDirectory: PathBuf,
}

impl Display {
    pub fn new() -> Self {
        Display {
            searchDirectory: PathBuf::new(),
        }
    }

    pub fn setDirectory(&mut self, directory: &Path) {
        self.searchDirectory = directory.to_path_buf();
    }
}

pub trait HandleDisplayEvent {
    fn displaySearchResult(&mut self, searchResult: (&Path, bool, &str));
}

impl HandleDisplayEvent for Display {
    fn displaySearchResult(&mut self, searchResult: (&Path, bool, &str)) {
        let (filePath, textFound, searchText) = searchResult;

        if textFound {
            print!("Text -> {:?} was found in file ->  {:?} ", searchText, filePath);
        } else {
            print!("Text -> {:?} was not found in file ->  {:?} ", searchText, filePath);
        }
    }
}

/*************************************Unit Test************************************** */
#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    #[test]
    fn testGenerateoutputStruct() {
        let _ = Display::new();
    }
}
