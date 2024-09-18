mod json;
mod pfs;
mod command;
mod build;
pub const VERSION: &str = "0.1.2";
fn main() {
    command::init();
}
