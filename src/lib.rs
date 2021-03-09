/*
    This library contains helper code for TextSearch
    TextSearch class can find a text in specified file and display it on screen
*/

#[allow(non_snake_case)]
mod OutputHandler;
mod text_search;

#[allow(unused_imports)]
pub use text_search::*;
#[allow(unused_imports)]
pub use OutputHandler::*;

/*************************************Unit Test************************************** */
#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
