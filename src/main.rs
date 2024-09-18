mod json;
mod pfs;
mod command;
mod build;
pub const VERSION: &str = "v0.1.2-beta.1";
fn main() {
    command::init();
}
