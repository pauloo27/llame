use gtk::gio::ActionEntry;
use gtk::prelude::*;
use gtk4 as gtk;

pub fn add_esc_keyboard_action(app: &gtk::Application, window: &gtk::ApplicationWindow) {
    let action_close = ActionEntry::builder("esc_close")
        .activate(|window: &gtk::ApplicationWindow, _, _| {
            window.close();
        })
        .build();

    window.add_action_entries([action_close]);
    app.set_accels_for_action("win.esc_close", &["Escape"]);
}
