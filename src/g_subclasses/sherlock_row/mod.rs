mod imp;

use gdk_pixbuf::subclass::prelude::ObjectSubclassIsExt;
use glib::Object;
use gtk4::glib;

glib::wrapper! {
    pub struct SherlockRow(ObjectSubclass<imp::SherlockRow>)
        @extends gtk4::ListBoxRow, gtk4::Widget;
}

impl SherlockRow {
    pub fn new() -> Self {
        Object::builder().build()
    }
    pub fn set_spawn_focus(&self, focus: bool) {
        self.imp().spawn_focus.set(focus);
    }
    pub fn set_shortcut(&self, shortcut: bool) {
        self.imp().shortcut.set(shortcut);
    }
}

impl Default for SherlockRow {
    fn default() -> Self {
        Self::new()
    }
}
