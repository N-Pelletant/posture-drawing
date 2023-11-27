use crate::posture_drawing_app_struct::SpeedDrawingApp;

impl SpeedDrawingApp {
    pub fn set_control_layout(&self) {
        self.hide_all_controls();
        self.center_window();

        self.directory_input.set_visible(true);
        self.choose_directory_button.set_visible(true);
        self.start_drawing_button.set_visible(true);
        self.timer_select_label.set_visible(true);
        self.timer_radio_00_30.set_visible(true);
        self.timer_radio_01_00.set_visible(true);
        self.timer_radio_02_00.set_visible(true);
        self.timer_radio_05_00.set_visible(true);
    }

    pub fn enable_start_drawing_btn(&self) {
        self.start_drawing_button.set_enabled(true);
    }
}
