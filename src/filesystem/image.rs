use std::ffi::OsString;

use nwg::ImageData;
use rand::seq::SliceRandom;

use crate::posture_drawing_app_struct::SpeedDrawingApp;

impl SpeedDrawingApp {
    pub fn create_random_indices_array(&self) {
        let mut rng = rand::thread_rng();

        let list_ref = self.image_list.borrow();

        let list = list_ref.clone().unwrap_or(vec![]);

        let mut randomized_list: Vec<usize> = (0..list.len()).collect();

        randomized_list.shuffle(&mut rng);

        self.image_indices.borrow_mut().replace(randomized_list);
    }

    pub fn display_new_image(&self) {
        let list_ref = self.image_list.borrow();

        let list = list_ref.clone().unwrap_or(vec![]);

        if list.len() == 0 {
            self.stop_chrono_timer();
            self.reset_chrono_timer_time_left();

            self.set_control_layout();
            return;
        }

        let mut indices = self
            .image_indices
            .take()
            .expect("To have an array of indices");

        let new_image_index = match indices.pop() {
            Some(index) => index,
            None => {
                self.stop_chrono_timer();
                self.reset_chrono_timer_time_left();

                self.set_control_layout();
                return;
            }
        };

        self.image_indices.borrow_mut().replace(indices);

        self.read_image(list.get(new_image_index).unwrap());
    }

    fn read_image(&self, filename: &OsString) {
        let filename = match filename.to_str() {
            Some(str) => str,
            None => panic!("This should not happen"),
        };

        let image = match self.decoder.from_filename(filename) {
            Ok(img) => img,
            Err(_) => {
                println!("Could not read image!");
                return;
            }
        };

        let frame = match image.frame(0) {
            Ok(bmp) => bmp,
            Err(_) => {
                println!("Could not read image frame!");
                return;
            }
        };

        match self.resize_if_necessary(&frame).as_bitmap() {
            Ok(bitmap) => {
                let mut img = self.loaded_image.borrow_mut();
                img.replace(bitmap);
                self.image_container.set_bitmap(img.as_ref());
            }
            Err(_) => {
                println!("Could not convert image to bitmap!");
            }
        }
    }

    fn resize_if_necessary(&self, frame: &ImageData) -> ImageData {
        let frame_size = frame.size();
        let image_width: f64 = frame_size.0.into();
        let image_height: f64 = frame_size.1.into();

        let container_size = self.image_container.size();
        let container_width: f64 = container_size.0.into();
        let container_height: f64 = container_size.1.into();

        let ratio = 1f64
            .max(image_width / container_width)
            .max(image_height / container_height);

        let new_sizes: [u32; 2] = [
            (image_width / ratio).round() as u32,
            (image_height / ratio).round() as u32,
        ];

        match self.decoder.resize_image(frame, new_sizes) {
            Ok(resized) => resized,
            Err(_) => {
                panic!("Could not resize image frame!");
            }
        }
    }
}
