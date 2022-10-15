use rocket::fs::TempFile;

#[derive(FromForm)]
pub struct PictureUpload<'r> {
    pub picture: TempFile<'r>,
}
