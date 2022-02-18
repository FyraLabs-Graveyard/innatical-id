use adw::prelude::*;

use adw::{ApplicationWindow};
use gtk::{Application, Stack};

use crate::gui::pages;

pub fn build_ui(app: &Application) {
    let builder = gtk::Builder::from_string(include_str!("window.ui"));

    let window: ApplicationWindow = builder
        .object("window")
        .expect("Could not get object `window` from builder.");
    let stack: Stack = builder
        .object("stack")
        .expect("Could not get object `stack` from builder.");

    window.set_application(Some(app));

    // stack.add_child(&pages::welcome::WelcomeCarousel::new());
    stack.add_child(&pages::settings::SettingsFlap::new());

    window.present();
}
