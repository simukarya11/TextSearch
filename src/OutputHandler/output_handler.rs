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

    pub fn setDirectory(&mut self, dir: &Path) {
        self.searchDirectory = dir.to_path_buf();
    }
}

pub trait HandleDisplayEvent {
    fn displaySearchResult(&mut self, result: (&Path, bool, &str));
}

impl HandleDisplayEvent for Display {
    fn displaySearchResult(&mut self, result: (&Path, bool, &str)) {
        let (filePath, textFound, searchText) = result;

        if textFound {
            print!("\n    {:?}: {:?} found", filePath, searchText);
        } else {
            print!("\n    {:?}: {:?} not found", filePath, searchText);
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
