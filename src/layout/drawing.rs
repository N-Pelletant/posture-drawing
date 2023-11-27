use crate::posture_drawing_app_struct::SpeedDrawingApp;

impl SpeedDrawingApp {
    pub fn set_drawing_layout(&self) {
        self.window.minimize();
        self.window.maximize();
        self.hide_all_controls();

        self.image_container.set_visible(true);
        self.back_to_control_btn.set_visible(true);
        self.time_left_label.set_visible(true);
        self.play_pause_btn.set_visible(true);
        self.skip_btn.set_visible(true);
    }
}
