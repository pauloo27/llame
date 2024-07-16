use gtk::glib;
use gtk::{gio, glib::clone};
use gtk4 as gtk;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::{self, get_icon_name_for_app};

use super::preview::Preview;

pub struct AppList {
    pub container: gtk::Box,
    window: gtk::ApplicationWindow,
    all_apps: Rc<Vec<Rc<gio::AppInfo>>>,
    preview: Rc<Preview>,
    apps_container: gtk::Box,
    displayed_apps: RefCell<Rc<Vec<Rc<gio::AppInfo>>>>,
}

impl AppList {
    pub fn new(window: gtk::ApplicationWindow, app_list: Vec<Rc<gio::AppInfo>>) -> AppList {
        let app_list = Rc::new(app_list);

        let main_container = gtk::Box::builder()
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

        let preview = Rc::new(Preview::new());

        preview_container.append(&preview.icon);
        preview_container.append(&preview.name);
        preview_container.append(&preview.description);

        let scrolled = gtk::ScrolledWindow::builder()
            .vexpand(true)
            .hexpand(true)
            .child(&apps_container)
            .build();

        main_container.append(&scrolled);
        main_container.append(&preview_container);

        let app_list = AppList {
            apps_container,
            window,
            preview,
            all_apps: app_list.clone(),
            displayed_apps: RefCell::new(app_list),
            container: main_container,
        };
        app_list.show_results();

        app_list
    }
}

impl AppList {
    pub fn filter_apps(&self, search_value: &str) {
        let search_value = search_value.to_lowercase();

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
            utils::launch_app(app).expect("Failed to launch app");
            self.window.set_visible(false);
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

            app_container.append(&gtk::Image::builder().icon_name(icon_name.clone()).build());
            app_container.append(&gtk::Label::builder().label(app.name()).build());

            let app_btn = gtk::Button::builder().child(&app_container).build();

            app_btn.connect_clicked(clone!(@strong app, @weak self.window as window => move |_|
                utils::launch_app(&app).expect("Failed to launch app");
                window.set_visible(false);
            ));

            app_btn.connect_has_focus_notify(clone!(@strong app, @strong preview => move |_| {
                preview.set_preview_for_app(&app);
            }));

            self.apps_container.append(&app_btn);
        }
    }
}
