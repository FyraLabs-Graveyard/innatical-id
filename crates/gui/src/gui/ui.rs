use adw::prelude::*;

use adw::{ApplicationWindow};
use gtk::{Application, Button};

pub fn build_ui(app: &Application) {
    let builder = gtk::Builder::from_string(include_str!("window.ui"));

    let window: ApplicationWindow = builder
        .object("window")
        .expect("Could not get object `window` from builder.");
    let button: Button = builder
        .object("button")
        .expect("Could not get object `button` from builder.");

    window.set_application(Some(app));

    button.connect_clicked(move |button| {
        button.set_label("Hello World!");
    });

    window.present();
}
