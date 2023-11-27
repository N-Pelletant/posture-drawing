use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::Path;

use crate::posture_drawing_app_struct::SpeedDrawingApp;

impl SpeedDrawingApp {
    pub fn choose_folder(&self) -> Option<String> {
        let current_folder = match env::current_dir() {
            Ok(current_folder) => current_folder,
            Err(e) => panic!("{e:#?}"),
        };

        let folder_string = match current_folder.to_str() {
            Some(folder_string) => folder_string,
            None => panic!("Failed to parsed current working directory to string"),
        };

        self.dialog
            .set_default_folder(folder_string)
            .expect("Failed to set default folder.");

        if !self.dialog.run(Some(&self.window)) {
            return None;
        }

        let directory_result = match self.dialog.get_selected_item() {
            Ok(directory) => directory.into_string(),
            Err(e) => panic!("{e:#?}"),
        };

        let dir = match directory_result {
            Ok(dir) => dir,
            Err(e) => panic!("{e:#?}"),
        };

        self.directory_input.set_text(&dir);

        Some(dir)
    }

    pub fn list_files(&self, directory: impl Into<String>) -> usize {
        let valid_image_extension = vec!["png", "jpg", "jpeg", "bmp", "webp"];

        let mut image_list = self.image_list.borrow_mut();

        let dir_content = match fs::read_dir(directory.into()) {
            Ok(read_dir) => read_dir,
            Err(e) => panic!("{e:#?}"),
        };

        let list: Vec<OsString> = dir_content
            .filter_map(|dir_entry_result| {
                let full_path = match dir_entry_result {
                    Ok(entry) => entry,
                    Err(_) => return None,
                };

                let filename = full_path.file_name();

                let path = Path::new(&filename);

                let file_extension = match path.extension().and_then(OsStr::to_str) {
                    Some(file_extension) => file_extension,
                    None => return None,
                };

                if !valid_image_extension.contains(&file_extension) {
                    return None;
                }

                Some(full_path.path().into_os_string())
            })
            .collect();

        let length = list.len();

        image_list.replace(list);

        length
    }
}
