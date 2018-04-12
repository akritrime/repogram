#[derive(Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub content: String
}

impl NewPost {
    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }
}