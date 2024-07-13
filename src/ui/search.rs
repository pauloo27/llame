use super::app_list::AppList;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk4 as gtk;
use std::rc::Rc;

pub fn build_search(app_list: Rc<AppList>) -> gtk::Entry {
    let search = gtk::Entry::builder()
        .primary_icon_name("system-search-symbolic")
        .build();

    search.connect_changed(clone!(@strong app_list => move |search| {
        let search_value = search.text();
        match search_value.as_str() {
            "" => app_list.remove_filter(),
            v => app_list.filter_apps(v),
        }
    }));

    search.connect_activate(clone!(@strong app_list => move |_| {
        app_list.launch_first();
    }));

    search
}
