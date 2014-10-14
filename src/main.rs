#![feature(globs)]

extern crate rgtk;

use rgtk::*;
use rgtk::gtk::signals;

fn append_text_column(tree: &mut gtk::TreeView) {
    let column = gtk::TreeViewColumn::new().unwrap();
    let cell = gtk::CellRendererText::new().unwrap();
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
}

fn add_row_via_wrapper(store: &gtk::ListStore) {
    let iter = gtk::TreeIter::new().unwrap();

    let ptr = store.get_pointer();
    println!("{}", ptr);

    store.append(&iter);

    let new_ptr = store.get_pointer();
    println!("{}", new_ptr);

    store.set_string(&iter, 0, "I'm a row");
}

fn main() {
    gtk::init();

    let mut window = gtk::Window::new(gtk::window_type::TopLevel).unwrap();
    window.set_window_position(gtk::window_position::Center);

    window.connect(signals::DeleteEvent::new(|_| {
        gtk::main_quit();
        true
    }));

    let mut tree = gtk::TreeView::new().unwrap();
    let column_types = vec![glib::ffi::g_type_string];
    let store = gtk::ListStore::new(column_types).unwrap();
    let model = store.get_model().unwrap();

    tree.set_model(&model);
    tree.set_headers_visible(false);

    append_text_column(&mut tree);

    add_row_via_wrapper(&store);

    window.add(&tree);
    window.show_all();
    gtk::main();
}
