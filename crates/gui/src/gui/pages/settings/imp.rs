use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(file = "settings.ui")]
pub struct SettingsFlap;

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for SettingsFlap {
    const NAME: &'static str = "SettingsFlap";
    type Type = super::SettingsFlap;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for SettingsFlap {}
impl WidgetImpl for SettingsFlap {}
impl BinImpl for SettingsFlap {}