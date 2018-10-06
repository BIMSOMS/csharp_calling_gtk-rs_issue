#![windows_subsystem = "windows"]

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;


fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    window.connect_delete_event(move |win, _| {
        win.destroy();
        Inhibit(false)
    });

    let button = gtk::Button::new_with_label("Click me!");

    window.add(&button);

    window.show_all();
}

#[no_mangle]
pub extern "C" fn get_it_done() -> u32{
    let application = gtk::Application::new("com.github.basic", gio::ApplicationFlags::empty())
        .expect("Initialization failed...");
    //unimplemented!();
    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
    101
}
#[no_mangle]
pub extern "C" fn add_numbers(number1: i32, number2: i32) -> i32 {
    println!("Hello from rust!");
    //let s = get_it_done();
    let n = number1 + number2;
    n
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
