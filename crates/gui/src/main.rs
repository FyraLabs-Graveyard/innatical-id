use adw::prelude::*;
use gtk::{Application};

pub mod gui;
use gui::ui;

fn main() {
    let application = Application::builder()
        .application_id("org.innatical.id.settings")
        .build();

    application.connect_startup(|_| {
        adw::init();
    });

    application.connect_activate(ui::build_ui);

    application.run();
}
