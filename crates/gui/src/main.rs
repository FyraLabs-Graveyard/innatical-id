use adw::prelude::*;
use gtk::{Application};

pub mod gui;
use gui::ui;
mod gresource;

fn main() {
    let _ = gresource::init();

    let application = Application::builder()
        .application_id("com.innatical.id.settings")
        .build();

    application.connect_startup(|_| {
        adw::init();
    });

    application.connect_activate(ui::build_ui);

    application.run();
}
