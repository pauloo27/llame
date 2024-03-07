use gtk::prelude::*;
use gtk::{gio, glib};
use gtk4 as gtk;

const APP_ID: &str = "cafe.ndo.Lame";

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(setup_ui);

    app.run()
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

    load_global_css();

    let apps = gio::AppInfo::all();

    let search = gtk::Entry::builder()
        .primary_icon_name("system-search-symbolic")
        .build();

    let apps_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let scrolled = gtk::ScrolledWindow::builder()
        .vexpand(true)
        .child(&apps_container)
        .build();

    for app in apps {
        let icon_name = match app.icon() {
            Some(i) => i
                .to_string()
                .map(|s| s.to_string())
                .unwrap_or("".to_string()),
            None => "dialog-question-symbolic".to_string(),
        };

        let app_container = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(5)
            .build();

        let icon = gtk::Image::builder().icon_name(icon_name).build();
        let lbl = gtk::Label::builder().label(app.name().to_string()).build();

        app_container.append(&icon);
        app_container.append(&lbl);

        apps_container.append(&app_container);
    }

    main_container.append(&search);
    main_container.append(&scrolled);

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .default_width(350)
        .default_height(200)
        .title("Lame")
        .child(&main_container)
        .build();

    window.present();
}

fn load_global_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(include_str!("./style.css"));

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
