
// Calc-In-Rust-Attempt - Attempt to do simple calculator in Rust, while starting with zero knowledge of Rust.
// Copyright (C) <2024>  <teamoor>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;


#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (260, 420), position: (300, 300), title: "Calc in Rust Attempt", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    #[nwg_control(text: "", size: (280, 25), position: (10, 10))]
    name_edit: nwg::TextInput,

    #[nwg_control(text: "7", size: (60, 60), position: (10, 160))] // first is horizontal, second is vertical
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    button2_1_button: nwg::Button,

    #[nwg_control(text: "4", size: (60, 60), position: (10, 220))] // first is horizontal, second is vertical
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    button3_1_button: nwg::Button,

    #[nwg_control(text: "1", size: (60, 60), position: (10, 280))] // first is horizontal, second is vertical
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    button4_1_button: nwg::Button,

    #[nwg_control(text: "8", size: (60, 60), position: (70, 160))] // first is horizontal, second is vertical
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    button2_2_button: nwg::Button,

    #[nwg_control(text: "5", size: (60, 60), position: (70, 220))] // first is horizontal, second is vertical
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    button3_2_button: nwg::Button,

    #[nwg_control(text: "2", size: (60, 60), position: (70, 280))] // first is horizontal, second is vertical
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    button4_2_button: nwg::Button,

    #[nwg_control(text: "0", size: (60, 60), position: (70, 340))] // first is horizontal, second is vertical
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    button5_2_button: nwg::Button,

    #[nwg_control(text: "⌫", size: (60, 60), position: (190, 40))] // first is horizontal, second is vertical
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    button_clear_button: nwg::Button,

    #[nwg_control(text: "стереть", size: (60, 60), position: (70, 40))] // first is horizontal, second is vertical
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    button_clear_current_button: nwg::Button,

    #[nwg_control(text: "9", size: (60, 60), position: (130, 160))] // first is horizontal, second is vertical
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    button2_3_button: nwg::Button,

    #[nwg_control(text: "6", size: (60, 60), position: (130, 220))] // first is horizontal, second is vertical
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    button3_3_button: nwg::Button,

    #[nwg_control(text: "3", size: (60, 60), position: (130, 280))] // first is horizontal, second is vertical
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    button4_3_button: nwg::Button,

    // #[nwg_control(text: "button5_5", size: (60, 60), position: (130, 280))] // first is horizontal, second is vertical
    // #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    // button5_3_button: nwg::Button,

    #[nwg_control(text: "/", size: (60, 60), position: (190, 100))] // first is horizontal, second is vertical
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    button_4_button: nwg::Button,

    #[nwg_control(text: "*", size: (60, 60), position: (190, 160))] // first is horizontal, second is vertical
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    button2_4_button: nwg::Button,

    #[nwg_control(text: "-", size: (60, 60), position: (190, 220))] // first is horizontal, second is vertical
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    button3_4_button: nwg::Button,

    #[nwg_control(text: "+", size: (60, 60), position: (190, 280))] // first is horizontal, second is vertical
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    button4_4_button: nwg::Button,

    #[nwg_control(text: "=", size: (60, 60), position: (190, 340))] // first is horizontal, second is vertical
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    button5_4_button: nwg::Button,
}

impl BasicApp {

    fn say_hello(&self) {
        // nwg::simple_message("Hello", &format!("Hello {}", self.name_edit.text()));
    }
    
    fn say_goodbye(&self) {
        // nwg::simple_message("Goodbye", &format!("Goodbye {}", self.name_edit.text()));
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}