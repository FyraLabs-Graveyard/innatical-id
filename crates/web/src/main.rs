extern crate gtk;
extern crate webkit2gtk;

use gtk::{prelude::*, Application, ApplicationWindow};
use webkit2gtk::{traits::*, WebContext, WebView};

fn main() {
    let app = Application::builder()
        .application_id("com.innatical.id.settings.Webkit")
        .build();

        app.connect_activate(|app| {
            let win = ApplicationWindow::builder()
                .application(app)
                .default_width(500)
                .default_height(600)
                .title("Innatical ID")
                .build();

            let context = WebContext::default().unwrap();
            let webview = WebView::with_context(&context);
            webview.load_uri("https://innatical.com");
            win.add(&webview);

            // todo make this work idk
            // webview.connect_load_changed(move |webview, signal| {
            //     if signal == LoadEvent::Finished {
            //         &win.set_title(&webview.title().unwrap());
            //     }
            // });

            win.show_all();
        });

    app.run();
}