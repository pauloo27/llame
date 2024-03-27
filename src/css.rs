use gtk4 as gtk;
use std::{fs, path::PathBuf, process};

pub fn load_css_from_file(path: PathBuf) {
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
