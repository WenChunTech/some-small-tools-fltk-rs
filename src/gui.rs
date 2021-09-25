use crate::win_tool::use_test;
use arboard::Clipboard;
use fltk::{
    dialog,
    button::*,
    enums::Align,
    group::{Group, Pack, Tabs},
    input::Input,
    menu::Choice,
    prelude::{GroupExt, MenuExt, WidgetBase, WidgetExt},
};
use fltk_flex::Flex;

pub fn draw_gallery() {
    let tab = Tabs::new(20, 30, 800 - 20, 600 - 20, "小工具合集");

    let grp1 = Group::new(40, 90, 800 - 40, 600 - 30, "Windows\t\t");
    // let mut flex = Flex::default().size_of_parent().column();

    let mut pack = Pack::new(40, 90, 800 - 40, 600 - 30, "在哪里");
    // pack.set_spacing(10);
    let mut copy_ssh_rsa_btn = Button::new(50, 100, 100, 40, "复制SSH公钥");
    let mut chcoice = Choice::new(60, 240, 200, 40, "选择组件");
    chcoice.set_label("请选择源");
    chcoice.add_choice("Hello");
    // flex.end();
    // let _inp = Input::default().with_size(0, 30).with_label("");

    pack.end();
    grp1.end();

    let grp2 = Group::new(40, 90, 800 - 40, 600 - 30, "Linux\t\t");
    grp2.end();
    tab.end();
    copy_ssh_rsa_btn.set_callback(|_| {
        let mut clipboard = Clipboard::new().unwrap();
        match clipboard.set_text(use_test()) {
            Ok(_) => {
                dialog::message(80, 60, "复制成功")
            }
            Err(_) => {
                dialog::alert_default("复制失败")
            }
        }
    });
}
