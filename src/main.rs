mod css;
mod keybinds;

use glib::clone;
use gtk::gio::{self};
use gtk::glib;
use gtk::pango;
use gtk::prelude::*;
use gtk4 as gtk;
use std::cell::RefCell;
use std::rc::Rc;
use std::{env, process};

const APP_ID: &str = "cafe.ndo.Llame";

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(setup_ui);

    let args: [String; 0] = [];
    app.run_with_args(&args)
}

struct Preview {
    icon: gtk::Image,
    name: gtk::Label,
    description: gtk::Label,
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

    let result_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .homogeneous(true)
        .build();

    let apps_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let preview_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(5)
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();

    let preview = Preview {
        icon: gtk::Image::builder()
            .icon_size(gtk::IconSize::Large)
            .build(),
        name: gtk::Label::builder().build(),
        description: gtk::Label::builder()
            .wrap(true)
            .wrap_mode(pango::WrapMode::WordChar)
            .build(),
    };
    let preview = Rc::new(preview);

    preview_container.append(&preview.icon);
    preview_container.append(&preview.name);
    preview_container.append(&preview.description);

    let scrolled = gtk::ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .child(&apps_container)
        .build();

    result_container.append(&scrolled);
    result_container.append(&preview_container);

    search.connect_changed(
        clone!(@strong all_apps, @strong displayed_apps, @weak apps_container, @strong preview => move |search| {
            let search_value = search.text().to_string().to_lowercase();

            if search_value == "" {
                show_apps(all_apps.clone(), apps_container, preview.clone());
                displayed_apps.replace(all_apps.clone());
            } else {
                let apps: Vec<Rc<gio::AppInfo>> = all_apps.iter()
                    .filter(|app| app.name().to_lowercase().contains(&search_value)).cloned().collect();

                let apps_rc = Rc::new(apps);
                show_apps(apps_rc.clone(), apps_container, preview.clone());
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

    show_apps(all_apps.clone(), apps_container, preview.clone());

    main_container.append(&search);
    main_container.append(&result_container);

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .default_width(550)
        .default_height(300)
        .title("Llame")
        .child(&main_container)
        .build();

    keybinds::add_esc_keyboard_action(&app, &window);

    window.present();
}

fn show_apps(apps: Rc<Vec<Rc<gio::AppInfo>>>, apps_container: gtk::Box, preview: Rc<Preview>) {
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

        let icon = gtk::Image::builder().icon_name(icon_name.clone()).build();
        let lbl = gtk::Label::builder().label(app.name()).build();

        app_container.append(&icon);
        app_container.append(&lbl);

        let app_btn = gtk::Button::builder().child(&app_container).build();

        app_btn.connect_clicked(clone!(@strong app => move |_| must_launch(&app)));

        app_btn.connect_has_focus_notify(clone!(@strong app, @strong preview => move |_| {
            preview.name.set_label(app.name().as_str());
            preview.description.set_label(&app.description().map(|s| s.to_string()).unwrap_or("".to_string()));
            preview.icon.set_icon_name(Some(&icon_name));
        }));

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
