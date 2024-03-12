use glib::clone;
use gtk::gio::{self, ActionEntry};
use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::{env, fs, process};

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
        load_css_from_file(path.into());
    }

    let all_apps: Rc<Vec<Rc<gio::AppInfo>>> = Rc::new(
        gio::AppInfo::all()
            .iter()
            .map(|a| Rc::new(a.clone()))
            .collect(),
    );
    // will change, when the search filters are applied
    let displayed_apps = Rc::new(RefCell::new(all_apps.clone()));

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

    search.connect_changed(
        clone!(@strong all_apps, @strong displayed_apps, @weak apps_container => move |search| {
            let search_value = search.text().to_string().to_lowercase();

            if search_value == "" {
                show_apps(all_apps.clone(), apps_container);
                displayed_apps.replace(all_apps.clone());
            } else {
                let apps: Vec<Rc<gio::AppInfo>> = all_apps.iter()
                    .filter(|app| app.name().to_lowercase().contains(&search_value)).cloned().collect();

                let apps_rc = Rc::new(apps);
                show_apps(apps_rc.clone(), apps_container);
                displayed_apps.replace(apps_rc);
            }
        }),
    );

    search.connect_activate(clone!(@strong displayed_apps => move |_| {
        let apps = displayed_apps.borrow();
        let top_result = apps.first();
        if let Some(top_result) = top_result {
            must_launch(top_result);
        }
    }));

    show_apps(all_apps.clone(), apps_container);

    main_container.append(&search);
    main_container.append(&scrolled);

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .default_width(350)
        .default_height(200)
        .title("Lame")
        .child(&main_container)
        .build();

    add_esc_keyboard_action(&app, &window);

    window.present();
}

fn show_apps(apps: Rc<Vec<Rc<gio::AppInfo>>>, apps_container: gtk::Box) {
    let mut next_child = apps_container.first_child();

    while let Some(child) = next_child {
        next_child = child.next_sibling();
        apps_container.remove(&child);
    }

    for app in apps.as_ref() {
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

        app_btn.connect_clicked(clone!(@strong app => move |_| must_launch(&app)));

        apps_container.append(&app_btn);
    }
}

fn must_launch(app: &gio::AppInfo) {
    match app.launch(&[], None::<&gio::AppLaunchContext>) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("Error {e}");
            process::exit(1);
        }
    };
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

fn add_esc_keyboard_action(app: &gtk::Application, window: &gtk::ApplicationWindow) {
    let action_close = ActionEntry::builder("esc_close")
        .activate(|window: &gtk::ApplicationWindow, _, _| {
            window.close();
        })
        .build();

    window.add_action_entries([action_close]);
    app.set_accels_for_action("win.esc_close", &["Escape"]);
}
