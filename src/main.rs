use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;

mod config;
mod ui;
mod utils;
use config::Config;

const APP_ID: &str = "cafe.ndo.Llame";

fn main() -> glib::ExitCode {
    let config = Config::load_from_file().ok();

    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui::setup_ui(config));

    app.run()
}
