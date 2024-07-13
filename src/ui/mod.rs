use gtk::prelude::*;
use gtk4 as gtk;
use std::{env, rc::Rc};

use crate::utils;

mod app_list;
mod css;
mod keybinds;
mod preview;
mod search;

pub fn setup_ui(app: &gtk::Application) {
    let main_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(10)
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();

    // TODO: load from config file instead of argv
    if let Some(path) = env::args().nth(1) {
        css::load_css_from_file(path.into());
    }

    let app_list = Rc::new(app_list::AppList::new(utils::load_apps()));
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
