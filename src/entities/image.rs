use rocket::fs::TempFile;
use rocket::serde::Serialize;

#[derive(FromForm)]
pub struct ImageUpload<'r> {
    pub image: TempFile<'r>,
}

#[derive(Serialize)]
pub struct ImageClassification {
    #[serde(rename(serialize = "isDog"))]
    pub is_dog: bool,
    #[serde(rename(serialize = "isCat"))]
    pub is_cat: bool,
    pub emoji: String,
}

impl ImageClassification {
    pub fn dog() -> Self {
        ImageClassification {
            is_dog: true,
            is_cat: false,
            emoji: String::from("🐶"),
        }
    }

    pub fn cat() -> Self {
        ImageClassification {
            is_dog: false,
            is_cat: true,
            emoji: String::from("🐱"),
        }
    }
}
