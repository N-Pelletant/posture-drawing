use nwg::RadioButtonState::Checked;
use std::time::Duration;

use crate::posture_drawing_app_struct::SpeedDrawingApp;

impl SpeedDrawingApp {
    pub fn get_duration_in_seconds(&self) -> u64 {
        if let Checked = self.timer_radio_00_30.check_state() {
            return 30;
        }

        if let Checked = self.timer_radio_01_00.check_state() {
            return 60;
        }

        if let Checked = self.timer_radio_02_00.check_state() {
            return 120;
        }

        if let Checked = self.timer_radio_05_00.check_state() {
            return 300;
        }

        return 30;
    }

    pub fn set_chrono_timer_duration(&self, d: u64) {
        self.duration.borrow_mut().replace(Duration::from_secs(d));
        self.time_left.borrow_mut().replace(d);
    }

    pub fn reset_chrono_timer_time_left(&self) {
        self.time_left
            .borrow_mut()
            .replace(self.get_duration_in_seconds());
    }

    pub fn update_time_left_label(&self, time_left: u64) {
        self.time_left_label
            .set_text(&format!("{:02}:{:02}", time_left / 60, time_left % 60));
    }

    pub fn decrement_count(&self) {
        let mut time_left_ref = self.time_left.borrow_mut();

        let time_left = match time_left_ref.clone() {
            Some(value) => value,
            None => 0,
        };

        time_left_ref.replace(time_left - 1);
    }

    pub fn start_chrono_timer(&self) {
        self.is_running.borrow_mut().replace(true);
        self.chrono_timer.start();
    }

    pub fn stop_chrono_timer(&self) {
        self.is_running.borrow_mut().replace(false);
        self.chrono_timer.stop();
    }

    pub fn toggle_chrono_timer(&self) {
        let is_running_ref = self.is_running.borrow();

        let is_running = is_running_ref.unwrap();

        drop(is_running_ref);

        match is_running {
            true => self.stop_chrono_timer(),
            false => self.start_chrono_timer(),
        }
    }
}
