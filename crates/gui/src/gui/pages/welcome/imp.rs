use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(file = "welcome.ui")]
pub struct WelcomeCarousel {
    #[template_child]
    pub lines_box: TemplateChild<adw::Bin>
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for WelcomeCarousel {
    const NAME: &'static str = "WelcomeCarousel";
    type Type = super::WelcomeCarousel;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for WelcomeCarousel {}
impl WidgetImpl for WelcomeCarousel {}
impl BinImpl for WelcomeCarousel {}