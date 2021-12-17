use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;

fn main() {
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    window.present();
}
