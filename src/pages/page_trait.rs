// Common page interface for all pages
use gtk4::prelude::*;
use gtk4::Widget;

pub trait Page {
    fn get_widget(&self) -> &gtk4::Widget;
    fn refresh(&mut self) {}
}
