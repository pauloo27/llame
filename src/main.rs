use std::path::PathBuf;
use std::rc::Rc;
use std::{env, fs, process};

use glib::clone;
use gtk::prelude::*;
use gtk::{gio, glib};
use gtk4 as gtk;

const APP_ID: &str = "cafe.ndo.Lame";

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(setup_ui);

    if let Some(path) = env::args().nth(1) {
        load_css_from_file(path.into());
    }

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

    let apps = Rc::new(gio::AppInfo::all());

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

    search.connect_changed(clone!(@strong apps, @weak apps_container => move |search| {
        let search_value = search.text().to_string().to_lowercase();

        if search_value == "" {
            show_apps(apps.clone(), apps_container);
        } else {
            // more clone omg
            let filtered_apps: Vec<gio::AppInfo> = apps.iter().filter(|app| app.name().to_lowercase().contains(&search_value)).cloned().collect();
            show_apps(Rc::new(filtered_apps), apps_container);
        }
    }));

    show_apps(apps.clone(), apps_container);

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

fn show_apps(apps: Rc<Vec<gio::AppInfo>>, apps_container: gtk::Box) {
    let mut next_child = apps_container.first_child();

    while let Some(child) = next_child {
        next_child = child.next_sibling();
        apps_container.remove(&child);
    }

    // FIXME: will clone, shit
    for app in apps.to_vec() {
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

        let app_btn = gtk::Button::builder().child(&app_container).build();

        app_btn.connect_clicked(move |_| {
            let _ = app.launch(&[], None::<&gio::AppLaunchContext>);
        });

        apps_container.append(&app_btn);
    }
}

fn load_css_from_file(path: PathBuf) {
    let provider = gtk::CssProvider::new();
    let data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Faile to load css file {err}");
            process::exit(1);
        }
    };
    provider.load_from_data(&data);

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
