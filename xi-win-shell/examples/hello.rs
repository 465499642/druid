// Copyright 2018 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate xi_win_shell;
extern crate direct2d;

use std::cell::RefCell;

use direct2d::math::*;

use xi_win_shell::menu::Menu;
use xi_win_shell::paint::PaintCtx;
use xi_win_shell::util;
use xi_win_shell::win_main;
use xi_win_shell::window::{WindowBuilder, WindowHandle, WinHandler};

#[derive(Default)]
struct HelloState {
    handle: RefCell<WindowHandle>,
}

impl WinHandler for HelloState {
    fn connect(&self, handle: &WindowHandle) {
        *self.handle.borrow_mut() = handle.clone();
    }

    fn paint(&self, paint_ctx: &mut PaintCtx) {
        let rt = paint_ctx.render_target();
        let size = rt.get_size();
        let rect = RectF::from((0.0, 0.0, size.width, size.height));
        let bg = rt.create_solid_color_brush(0x272822,
            &BrushProperties::default()).unwrap();
        rt.fill_rectangle(&rect, &bg);
        let fg = rt.create_solid_color_brush(0xf0f0ea,
            &BrushProperties::default()).unwrap();
        rt.draw_line(&Point2F::from((10.0, 50.0)), &Point2F::from((90.0, 90.0)),
                &fg, 1.0, None);
    }

    fn command(&self, id: u32) {
        match id {
            0x100 => self.handle.borrow().close(),
            _ => println!("unexpected id {}", id),
        }
    }

    fn char(&self, ch: u32) {
        println!("got char 0x{:x}", ch);
    }

    fn keydown(&self, vk_code: i32) {
        println!("got key code 0x{:x}", vk_code);
    }

    fn destroy(&self) {
        win_main::request_quit();
    }
}

fn main() {
    xi_win_shell::init();

    let mut file_menu = Menu::new();
    file_menu.add_item(0x100, "E&xit");
    let mut menubar = Menu::new();
    menubar.add_dropdown(file_menu, "&File");

    let mut builder = WindowBuilder::new();
    builder.set_handler(Box::new(HelloState::default()));
    builder.set_title("Hello example");
    builder.set_menu(menubar);
    let window = builder.build().unwrap();
    window.show();
    let mut run_loop = win_main::RunLoop::new();
    run_loop.run();
}
