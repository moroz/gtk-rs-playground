use custom_button::CustomButton;
use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;

pub mod custom_button;

fn main() {
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let button = CustomButton::new();
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_end(12);
    button.set_margin_start(12);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .default_width(120)
        .default_height(120)
        .build();

    window.present();
}
