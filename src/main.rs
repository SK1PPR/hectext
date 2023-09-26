#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]

mod editor;
use editor::Editor;
mod terminal;
pub use editor::Position;
use terminal::Terminal;
mod document;
use document::Document;
mod row;
use row::Row;

fn main() {
    Editor::default().run();
}
