mod apps;
mod css;
mod keybinds;
mod list;
mod search;

use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;
use std::env;
use std::rc::Rc;

const APP_ID: &str = "cafe.ndo.Llame";

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(setup_ui);

    let args: [String; 0] = [];
    app.run_with_args(&args)
}

fn setup_ui(app: &gtk::Application) {
    let main_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(10)
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();

    if let Some(path) = env::args().nth(1) {
        css::load_css_from_file(path.into());
    }

    let app_list = Rc::new(list::AppList::new(apps::load_apps()));
    let search_container = search::build_search(app_list.clone());

    main_container.append(&search_container);
    main_container.append(&app_list.container);

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .default_width(550)
        .default_height(300)
        .title("Llame")
        .child(&main_container)
        .build();

    keybinds::add_esc_keyboard_action(app, &window);

    window.present();
}
