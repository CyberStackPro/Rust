extern crate glib;
extern crate gtk;
extern crate sysinfo;

mod system_info;
use glib::{ControlFlow, clone};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Label};
use std::cell::RefCell;
use std::rc::Rc;
use system_info::SystemInfo;

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);
    window.set_title("System Monitor");
    window.set_default_size(350, 70);
    let vbox = Box::new(gtk::Orientation::Vertical, 10);
    let cpu_label = Label::new(Some("CPU Usage:"));
    let memory_label = Label::new(Some("Memory Usage:"));
    vbox.add(&cpu_label);
    vbox.add(&memory_label);
    window.add(&vbox);
    window.show_all();
    let system_info = Rc::new(RefCell::new(SystemInfo::new()));
    glib::timeout_add_seconds_local(
        1,
        clone!(@strong system_info, @strong cpu_label, @strong memory_label => move || {
            let system_info = system_info.borrow();
            cpu_label.set_text(&format!("CPU Usage: {:.2}%", system_info.get_cpu_usage()));
            let (used_memory, total_memory) = system_info.get_memory_usage();
            memory_label.set_text(&format!("Memory Usage: {} / {}", used_memory, total_memory));
            ControlFlow::Continue
        }),
    );
}

fn main() {
    let application = Application::new(Some("com.example.system_monitor"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}
