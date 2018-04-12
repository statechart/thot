#[macro_use]
extern crate serde_derive;

extern crate serde;

#[path = "ast"]
pub mod ast {
    #[path = "statechart.rs"]
    pub mod statechart;

    #[path = "core.rs"]
    pub mod core;

    #[path = "microstep.rs"]
    pub mod microstep;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
