use serde::Serialize;

#[derive(Serialize)]
pub struct Emoji {
    type_: String,
    emoji: String,
}

impl Emoji {
    pub fn new(emoji: String) -> Self {
        Self {
            type_: "emoji".to_string(),
            emoji: emoji,
        }
    }
}
