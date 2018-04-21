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

    #[path = "position.rs"]
    pub mod location;

    #[path = "conversion_error.rs"]
    pub mod conversion_error;
}
