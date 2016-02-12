use article::Article;
use homepage;
use gtk;
use gtk::traits::*;
use gtk::signal::WidgetSignals;
use gdk::ffi::GdkRGBA;
use pango;

fn configure_window(window: &gtk::Window) {
    window.set_title("Phoronix Reader");
    let (width, height) = (600, 500);
    window.set_default_size(width, height);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        gtk::signal::Inhibit(true)
    });
}

pub fn launch() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
    let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
    configure_window(&window);
    
    window.show_all();
    
    gtk::main();
}