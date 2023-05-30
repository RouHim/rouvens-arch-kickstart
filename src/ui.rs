use gtk4::glib;
use gtk4::glib::clone;
use gtk4::prelude::*;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual};
use gtk4::Application;

use std::sync::{Arc, Mutex};

use crate::shell::RootShell;
use crate::Feature;

dyn_clone::clone_trait_object!(Feature);

pub fn show(root_shell: RootShell, features: Vec<Box<dyn Feature>>) {
    let root_shell: Arc<Mutex<RootShell>> = Arc::new(Mutex::new(root_shell));

    let application = Application::new(
        Some("com.github.gtk4-rs.examples.basic"),
        Default::default(),
    );

    application.connect_activate(move |app| {
        let cloned_features = features.to_vec();
        build_ui(app, &root_shell, cloned_features);
    });

    application.run();
}

fn build_ui(
    application: &Application,
    root_shell: &Arc<Mutex<RootShell>>,
    features: Vec<Box<dyn Feature>>,
) {
    let window = gtk4::ApplicationWindow::new(application);

    window.set_title(Some("Rouvens arch kickstart"));
    window.set_default_size(480, 720);

    let headerbar = gtk4::HeaderBar::new();
    window.set_titlebar(Some(&headerbar));

    let content = gtk4::Box::new(gtk4::Orientation::Vertical, 0);

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_hexpand(true);
    scroll.set_child(Some(&content));

    window.set_child(Some(&scroll));

    for feature in features {
        let name = feature.get_name();
        let is_installed = feature.is_installed();

        if feature.is_group_element() {
            let group_header = gtk4::Label::new(Some(&name));
            group_header.set_hexpand(true);
            group_header.set_margin_top(10);
            group_header.set_margin_bottom(10);
            group_header.set_margin_start(10);
            group_header.set_margin_end(10);
            group_header.set_xalign(0.0);

            content.append(&group_header);
        } else {
            // Clone the feature
            let feature_clone: Arc<Mutex<Box<dyn Feature>>> = Arc::new(Mutex::new(feature));

            // Install button
            let btn_install = gtk4::Button::with_label(&name);
            btn_install.set_hexpand(true);
            btn_install.set_sensitive(!is_installed);

            // Uninstall button
            let btn_uninstall = gtk4::Button::with_label("X");
            btn_uninstall.set_sensitive(is_installed);

            btn_install.connect_clicked(
                clone!(@weak btn_install, @weak btn_uninstall, @weak root_shell, @strong feature_clone => move |_| {
                    let mut root_shell = root_shell.lock().unwrap();
                    let feature = feature_clone.lock().unwrap();

                    feature.install(&mut root_shell);

                    let feature_installed = feature.is_installed();
                    btn_install.set_sensitive(!feature_installed);
                    btn_uninstall.set_sensitive(feature_installed);
                }),
            );

            btn_uninstall.connect_clicked(
                clone!(@weak btn_uninstall, @weak btn_install , @weak root_shell, @strong feature_clone => move |_| {
                    let mut root_shell = root_shell.lock().unwrap();
                    let feature = feature_clone.lock().unwrap();
                    feature.uninstall(&mut root_shell);

                    let feature_installed = feature.is_installed();
                    btn_install.set_sensitive(!feature_installed);
                    btn_uninstall.set_sensitive(feature_installed);
                }),
            );

            // Horizontal box for the buttons
            let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 5);

            // hbox fill line
            hbox.append(&btn_install);
            hbox.append(&btn_uninstall);

            content.append(&hbox);
        }
    }

    window.present();
}
