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

        let dir = std::fs::read_dir(&self.path).unwrap();
        let random_file = dir.choose(&mut rng).unwrap();

        match random_file {
            Ok(entry) => Some(std::fs::read(entry.path()).unwrap()),
            Err(_) => None,
        }
    }
}
