use crate::posture_drawing_app_struct::{SpeedDrawingApp, HEIGHT, WIDTH};

impl SpeedDrawingApp {
    pub fn init_layout(&self) {
        self.hide_all_controls();

        let (x_pos, y_pos) = self.window.position();

        self.x_pos.borrow_mut().replace(x_pos);
        self.y_pos.borrow_mut().replace(y_pos);

        self.set_control_layout();
    }

    pub fn hide_all_controls(&self) {
        self.directory_input.set_visible(false);
        self.choose_directory_button.set_visible(false);
        self.timer_select_label.set_visible(false);
        self.timer_radio_00_30.set_visible(false);
        self.timer_radio_01_00.set_visible(false);
        self.timer_radio_02_00.set_visible(false);
        self.timer_radio_05_00.set_visible(false);
        self.start_drawing_button.set_visible(false);

        self.image_container.set_visible(false);
        self.back_to_control_btn.set_visible(false);
        self.time_left_label.set_visible(false);
        self.play_pause_btn.set_visible(false);
        self.skip_btn.set_visible(false);
    }

    pub fn center_window(&self) {
        let x_pos = match self.x_pos.clone().take() {
            Some(x) => x,
            None => 0,
        };
        let y_pos = match self.y_pos.clone().take() {
            Some(y) => y,
            None => 0,
        };
        self.window.set_position(x_pos, y_pos);
        self.window.set_size(WIDTH, HEIGHT);
    }
}
