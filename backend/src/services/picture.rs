pub struct PictureService {
    path: String
}

impl PictureService {
    pub fn new(path: String) -> PictureService {
        PictureService {
            path
        }
    }

    pub fn get_picture_url(self, host: String) -> String {
        host + ""
    }
}