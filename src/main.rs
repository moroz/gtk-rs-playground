use custom_button::CustomButton;
use gtk::glib;
use gtk::glib::*;
use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;
use gtk::Orientation;
use std::cell::Cell;
use std::rc::Rc;

pub mod custom_button;

fn main() {
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_button(text: &str) -> CustomButton {
    let button = CustomButton::with_label(text);
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_end(12);
    button.set_margin_start(12);
    button
}

fn build_ui(app: &Application) {
    let button_decrease = build_button("Decrease");
    let button_increase = build_button("Increase");

    let number = Rc::new(Cell::new(0));

    button_increase.connect_clicked(clone!(@weak number, @weak button_decrease => move |_| {
        number.set(number.get() + 1);
        button_decrease.set_label(&number.get().to_string());
    }));

    button_decrease.connect_clicked(clone!(@weak button_increase =>
    move |_| {
        number.set(number.get() - 1);
        button_increase.set_label(&number.get().to_string());
    }));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    let gtk_box = gtk::Box::new(Orientation::Vertical, 0);
    window.set_child(Some(&gtk_box));
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);

    window.present();
}
