use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(file = "welcome.ui")]
pub struct WelcomeCarousel {
    #[template_child]
    pub lines_box: TemplateChild<adw::Bin>,
    #[template_child]
    pub welcome_home: TemplateChild<adw::StatusPage>
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

impl ObjectImpl for WelcomeCarousel {
    fn constructed(&self, obj: &Self::Type) {
        // Call "constructed" on parent
        self.parent_constructed(obj);

        // Connect to "clicked" signal of `button`
        self.welcome_home.set_paintable(gtk::Image::from_resource("/org/innatical/id/settings/icons/earth-americas").paintable().as_ref())
    }
}
impl WidgetImpl for WelcomeCarousel {}
impl BinImpl for WelcomeCarousel {}