use gtk::gio;
use gtk::prelude::*;
use gtk4 as gtk;
use std::process;
use std::rc::Rc;

pub fn must_launch(app: &gio::AppInfo) {
    match app.launch(&[], None::<&gio::AppLaunchContext>) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("Error {e}");
            process::exit(1);
        }
    };
}

pub fn load_apps() -> Vec<Rc<gio::AppInfo>> {
    gio::AppInfo::all()
        .iter()
        .map(|a| Rc::new(a.clone()))
        .collect()
}
