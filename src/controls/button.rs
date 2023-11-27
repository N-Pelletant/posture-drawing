use crate::posture_drawing_app_struct::SpeedDrawingApp;

impl SpeedDrawingApp {
    pub fn choose_directory_click(&self) {
        if let Some(path) = self.choose_folder() {
            self.list_files(path);
            self.enable_start_drawing_btn();
        };
    }

    pub fn timer_radio_click(&self) {
        let d = self.get_duration_in_seconds();
        self.set_chrono_timer_duration(d);
    }

    pub fn start_drawing_click(&self) {
        self.set_drawing_layout();
        self.create_random_indices_array();
        self.display_new_image();
        self.start_chrono_timer();
    }

    pub fn back_to_control_click(&self) {
        self.stop_chrono_timer();
        self.reset_chrono_timer_time_left();

        self.set_control_layout();
    }

    pub fn play_pause_click(&self) {
        self.toggle_chrono_timer();
    }

    pub fn skip_click(&self) {
        self.reset_chrono_timer_time_left();
        self.display_new_image();
    }
}
