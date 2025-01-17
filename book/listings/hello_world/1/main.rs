use gtk::prelude::*;
use gtk::Application;

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk.example")
        .build();

    // Run the application
    app.run();
}
