use anyhow::Result as AnyResult;
use gtk4 as gtk;
use std::{fs, path::PathBuf};

pub fn load_css_from_file(path: &PathBuf) -> AnyResult<()> {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(&fs::read_to_string(path)?);

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    Ok(())
}
