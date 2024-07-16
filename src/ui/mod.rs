use crate::{config::Config, utils};
use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;
use std::rc::Rc;

mod app_list;
mod css;
mod keybinds;
mod preview;
mod search;

pub fn setup_ui(config: Option<Config>) -> impl Fn(&gtk::Application) + 'static {
    move |app: &gtk::Application| {
        let main_container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(10)
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();

        if let Some(css_file_path) = config.as_ref().and_then(|c| c.css_file_path.as_ref()) {
            css::load_css_from_file(css_file_path).expect("Failed to load CSS file");
        }

        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .default_width(550)
            .default_height(300)
            .title("Llame")
            .child(&main_container)
            .build();

        window.connect_close_request(|win| {
            win.set_visible(false);
            glib::Propagation::Stop
        });

        let app_list = Rc::new(app_list::AppList::new(window.clone(), utils::load_apps()));
        let search_container = search::build_search(app_list.clone());

        main_container.append(&search_container);
        main_container.append(&app_list.container);

        keybinds::add_esc_keyboard_action(app, &window);

        window.present();
    }
}
