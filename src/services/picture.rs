use std::fs;
use std::io::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use rand::seq::IteratorRandom;
use rand::thread_rng;

pub struct PictureService {
    path: String,
}

impl PictureService {
    pub fn new(path: String) -> PictureService {
        PictureService { path }
    }

    pub fn get_picture(&self) -> Option<Vec<u8>> {
        let mut rng = thread_rng();

        let dir = fs::read_dir(&self.path).unwrap();
        let random_file = dir.choose(&mut rng).unwrap();

        match random_file {
            Ok(entry) => Some(fs::read(entry.path()).unwrap()),
            Err(_) => None,
        }
    }

    pub fn save_picture(&self, data: &[u8]) -> Result<(), Error> {
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

        let mut picture_path = since_the_epoch.as_millis().to_string() + ".jpg";
        picture_path.insert_str(0, &self.path);

        fs::write(picture_path, data)
    }
}
