use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;

mod ui;
mod utils;

const APP_ID: &str = "cafe.ndo.Llame";

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui::setup_ui);

    let args: [String; 0] = [];
    app.run_with_args(&args)
}
