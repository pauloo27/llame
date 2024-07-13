use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;
use gtk4::gio;
use std::process;
use std::rc::Rc;

const DEFAULT_APP_ICON: &str = "dialog-question-symbolic";

pub fn get_icon_name_for_app(app: &gio::AppInfo) -> glib::GString {
    app.icon()
        .and_then(|i| i.to_string())
        .unwrap_or(DEFAULT_APP_ICON.into())
}

pub fn load_apps() -> Vec<Rc<gio::AppInfo>> {
    gio::AppInfo::all()
        .iter()
        .map(|a| Rc::new(a.clone()))
        .collect()
}

pub fn must_launch(app: &gio::AppInfo) {
    match app.launch(&[], None::<&gio::AppLaunchContext>) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("Error {e}");
            process::exit(1);
        }
    };
}
