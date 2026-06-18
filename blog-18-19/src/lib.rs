// Create the Post struct
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn listing_18_19() {
        let mut post = Post::new();

        post.add_text("Lorem ipsum verde broncochio");
        // assert_eq!("", post.content()); <-- No method, can't even compile
    }
}
