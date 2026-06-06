#![warn(clippy::all, rust_2018_idioms)]

pub(crate) mod app;
pub(crate) mod chart;
pub(crate) mod file_dialog;
pub(crate) mod futures;
pub(crate) mod midi;
pub(crate) mod project;
pub use app::WeaverApp;
