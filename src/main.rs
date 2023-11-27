#![windows_subsystem = "windows"]

mod controls;
mod filesystem;
mod layout;
mod posture_drawing_app_struct;
mod timers;

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwg::NativeUi;

use posture_drawing_app_struct::SpeedDrawingApp;

fn main() {
    nwg::init().expect("Failed to init Native windows gui");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let app = SpeedDrawingApp::build_ui(Default::default()).expect("Failed to build UI");
    app.init();

    nwg::dispatch_thread_events();
}
