use adw::prelude::*;

use adw::{ApplicationWindow, HeaderBar};
use gtk::{Application, Box, Orientation, Label};

fn main() {
    let application = Application::builder()
        .application_id("org.innatical.id.settings")
        .build();

    application.connect_startup(|_| {
        adw::init();
    });

    application.connect_activate(|app| {
        let label = Label::new(Some("Hello World"));

        let content = Box::new(Orientation::Vertical, 0);
        content.append(
            &HeaderBar::builder()
                .title_widget(&adw::WindowTitle::new("Innatical ID", ""))
                .build(),
        );
        content.append(&label);

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(350)
            .content(&content)
            .build();
        window.show();
    });

    application.run();
}
