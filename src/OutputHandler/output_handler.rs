use std::path::{Path, PathBuf};

/*----------Struct Display------------------------------------------*/

#[allow(dead_code)]
pub struct Display {
    searchDirectory: PathBuf,
}

pub trait OutputTrait {
    fn setDirectory(&mut self, directory: &Path);
    fn displaySearchResult(&mut self, searchResult: (&Path, bool, &str));
}
/*_________________Uncomment to use DisplayImplementation____________________ */
/*
impl Display {
    pub fn new() -> Self {
        Display {
            searchDirectory: PathBuf::new(),
        }
    }
}

impl OutputTrait for Display {
    fn setDirectory(&mut self, directory: &Path) {
        self.searchDirectory = directory.to_path_buf();
    }
    fn displaySearchResult(&mut self, searchResult: (&Path, bool, &str)) {
        let (filePath, textFound, searchText) = searchResult;

        if textFound {
            print!(
                "Text -> {:?} was found in file ->  {:?} ",
                searchText, filePath
            );
        } else {
            print!(
                "Text -> {:?} was not found in file ->  {:?} ",
                searchText, filePath
            );
        }
    }
}
*/

