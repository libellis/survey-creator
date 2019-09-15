#[macro_use]
extern crate domain_derive;

#[macro_use]
extern crate serde_derive;

pub mod inputs;
pub mod commands;
pub mod config;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
