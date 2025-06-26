pub trait Page {
    fn get_widget(&self) -> &gtk4::Widget;
    fn refresh(&mut self) {}
}
