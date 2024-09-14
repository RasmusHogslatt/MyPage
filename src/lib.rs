#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod constants;
mod education;
mod experience;
mod info;
mod project;
pub use app::PersonalPortfolio;
pub use constants::*;
pub use education::*;
pub use experience::*;
pub use info::*;
pub use project::*;
