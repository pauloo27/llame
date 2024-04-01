use crate::apps;
use glib::clone;
use gtk::gio;
use gtk::glib;
use gtk::pango;
use gtk::prelude::*;
use gtk4 as gtk;
use std::cell::RefCell;
use std::rc::Rc;

const DEFAULT_APP_ICON: &str = "dialog-question-symbolic";

pub struct Preview {
    icon: gtk::Image,
    name: gtk::Label,
    description: gtk::Label,
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

pub struct AppList {
    pub container: gtk::Box,
    all_apps: Rc<Vec<Rc<gio::AppInfo>>>,
    preview: Rc<Preview>,
    apps_container: gtk::Box,
    displayed_apps: Rc<RefCell<Rc<Vec<Rc<gio::AppInfo>>>>>,
}

impl AppList {
    pub fn new(app_list: Vec<Rc<gio::AppInfo>>) -> AppList {
        let app_list = Rc::new(app_list);

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

        let preview = Preview::new();
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

        let result = AppList {
            all_apps: app_list.clone(),
            apps_container,
            preview,
            displayed_apps: Rc::new(RefCell::new(app_list)),
            container: result_container,
        };
        result.show_results();

        result
    }
}

impl AppList {
    pub fn filter_apps(&self, search_value: &str) {
        let apps: Vec<Rc<gio::AppInfo>> = self
            .all_apps
            .iter()
            .filter(|app| app.name().to_lowercase().contains(&search_value))
            .cloned()
            .collect();

        self.displayed_apps.replace(Rc::new(apps));
        self.show_results();
    }

    pub fn remove_filter(&self) {
        self.displayed_apps.replace(self.all_apps.clone());
        self.show_results();
    }

    pub fn launch_first(&self) {
        if let Some(app) = self.displayed_apps.borrow().first() {
            apps::must_launch(app);
        }
    }
}

impl AppList {
    fn show_results(&self) {
        // clear the container
        let mut next_child = self.apps_container.first_child();
        while let Some(child) = next_child {
            next_child = child.next_sibling();
            self.apps_container.remove(&child);
        }

        // borrow some stuff
        let apps = self.displayed_apps.borrow();
        let preview = self.preview.clone();

        // by default, show the first app in the preview
        match apps.first() {
            Some(app) => preview.set_preview_for_app(app),
            None => preview.reset(),
        }

        for app in apps.as_ref() {
            let icon_name = get_icon_name_for_app(app);

            // creating a Button(Container(Icon, Label))
            let app_container = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .spacing(5)
                .build();

            let icon = gtk::Image::builder().icon_name(icon_name.clone()).build();
            let lbl = gtk::Label::builder().label(app.name()).build();

            app_container.append(&icon);
            app_container.append(&lbl);

            let app_btn = gtk::Button::builder().child(&app_container).build();

            app_btn.connect_clicked(clone!(@strong app => move |_| apps::must_launch(&app)));

            app_btn.connect_has_focus_notify(clone!(@strong app, @strong preview => move |_| {
                preview.set_preview_for_app(&app);
            }));

            self.apps_container.append(&app_btn);
        }
    }
}

fn get_icon_name_for_app(app: &gio::AppInfo) -> String {
    app.icon()
        .map(|i| i.to_string().unwrap_or("".into()).to_string())
        .unwrap_or(DEFAULT_APP_ICON.into())
}
