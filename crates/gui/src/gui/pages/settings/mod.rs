mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct SettingsFlap(ObjectSubclass<imp::SettingsFlap>)
        @extends gtk::Widget,
        @implements adw::Swipeable, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl SettingsFlap {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `SettingsFlap`.")
    }
}