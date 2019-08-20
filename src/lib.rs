#![recursion_limit="256"]
extern crate self as yewstrap;

pub mod components;

pub mod prelude {
    pub use crate::components::*;
}

pub use self::prelude::*;