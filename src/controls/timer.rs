use crate::SpeedDrawingApp;

impl SpeedDrawingApp {
    pub fn chrono_timer_tick(&self) {
        let time_left_ref = self.time_left.borrow();

        let time_left = match time_left_ref.clone() {
            Some(value) => value,
            None => 0,
        };

        self.update_time_left_label(time_left.try_into().unwrap());

        drop(time_left_ref);

        if time_left != 0 {
            self.decrement_count();
        } else {
            self.display_new_image();
            self.reset_chrono_timer_time_left();
        }
    }
}
