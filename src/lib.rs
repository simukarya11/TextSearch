/*
    This code is property of Simuk Arya
    Handle with care
    The library contains helper code for TextSearch
    TextSearch class can find a text in specified file and display it on screen using Trait object for OutputTrait
*/

#![allow(non_snake_case)]
#![allow(unused_imports)]

mod OutputHandler;
mod text_search;

use DirectoryNavigator;
pub use text_search::*;
pub use OutputHandler::*;

/*************************************Unit Test************************************** */
#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
