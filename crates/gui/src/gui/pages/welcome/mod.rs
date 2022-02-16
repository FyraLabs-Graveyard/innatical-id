mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct WelcomeCarousel(ObjectSubclass<imp::WelcomeCarousel>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl WelcomeCarousel {
    pub fn new() -> Self {
        // Create new window
        Object::new(&[]).expect("Failed to create `WelcomeCarousel`.")
    }
}