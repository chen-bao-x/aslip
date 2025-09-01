// aslip
pub use ascp_macro;
pub use ascp_macro::command;
pub use ascp_macro::run;

pub mod app;
pub mod type_casting;

use crate::app::App;

pub fn run() {
    App::new("").run();
}
