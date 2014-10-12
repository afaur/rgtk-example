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

fn add_row_via_ffi(store: &gtk::ListStore) {
    let mut iter = gtk::ffi::C_GtkTreeIter;

    unsafe {
        let ptr = store.get_pointer();
        println!("{}", ptr); // valid

        gtk::ffi::gtk_list_store_append(ptr, &mut iter);

        let ptr = store.get_pointer();
        println!("{}", ptr); // invalid

        gtk::ffi::gtk_list_store_set(ptr, &mut iter, 0i, "I'm a row".to_c_str().unwrap(), -1i);
    }
}

fn add_row_via_wrapper(store: &gtk::ListStore) {
    let mut iter_raw = gtk::ffi::C_GtkTreeIter;
    let iter = gtk::TreeIter::wrap_pointer(&mut iter_raw);

    let ptr = store.get_pointer();
    println!("{}", ptr); // valid

    store.append(&iter);

    let ptr = store.get_pointer();
    println!("{}", ptr); // invalid

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

    // both of these fail, but work fine when their
    // contents are moved into the main function
    add_row_via_ffi(&store);
    add_row_via_wrapper(&store);

    window.add(&tree);
    window.show_all();
    gtk::main();
}
