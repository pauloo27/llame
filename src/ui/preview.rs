use crate::utils::get_icon_name_for_app;
use gtk::{gio, glib, pango};
use gtk4 as gtk;
use gtk4::prelude::*;

pub struct Preview {
    pub icon: gtk::Image,
    pub name: gtk::Label,
    pub description: gtk::Label,
}

impl Preview {
    pub fn new() -> Preview {
        Preview {
            icon: gtk::Image::builder()
                .icon_size(gtk::IconSize::Large)
                .build(),
            name: gtk::Label::builder().build(),
            description: gtk::Label::builder()
                .wrap(true)
                .wrap_mode(pango::WrapMode::WordChar)
                .build(),
        }
    }

    pub fn reset(&self) {
        self.name.set_label("");
        self.icon.set_icon_name(None);
        self.description.set_label("");
    }

    pub fn set_preview_for_app(&self, app: &gio::AppInfo) {
        let name = app.name();
        let description = &app.description().unwrap_or(glib::GString::from(""));
        let icon_name = get_icon_name_for_app(app);

        self.name.set_label(&name);
        self.description.set_label(description);
        self.icon.set_from_icon_name(Some(&icon_name));
    }
}
