#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod tic_tac_toe;
pub(crate) use app::AppWindow;
pub use app::MainScreen;
