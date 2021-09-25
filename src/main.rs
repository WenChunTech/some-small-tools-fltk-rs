mod gui;
mod win_tool;
use crate::gui::draw_gallery;
use crate::win_tool::use_test;
use fltk::{
    app,
    prelude::{GroupExt, WidgetExt},
    window::Window,
};
fn main() {
    use_test();
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    app::background(221, 221, 221);
    let mut wind = Window::default().with_size(800, 600).with_label("小工具");
    draw_gallery();
    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}
