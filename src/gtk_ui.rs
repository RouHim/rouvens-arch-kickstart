use std::collections::HashMap;

use gtk4::{Application, ApplicationWindow};
use gtk4::glib;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual};
use gtk4::prelude::*;

use crate::Feature;
use crate::shell::RootShell;

pub fn show(root_shell: RootShell, features: Vec<Box<dyn Feature>>) {
    let application =
        gtk4::Application::new(Some("com.github.gtk4-rs.examples.basic"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(application);

    window.set_title(Some("First GTK Program"));
    window.set_default_size(350, 70);

    let button = gtk4::Button::with_label("Click me!");

    window.set_child(Some(&button));

    window.present();
}