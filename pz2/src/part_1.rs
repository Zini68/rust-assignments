use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub struct New;

#[derive(Debug, PartialEq)]
pub struct Unmoderated;

#[derive(Debug, PartialEq)]
pub struct Published;

#[derive(Debug, PartialEq)]
pub struct Deleted;

#[derive(Debug)]
pub struct Post<State> {
    content: String,
    state: PhantomData<State>,
}

impl Post<New> {
    pub fn new(content: &str) -> Self {
        Post {
            content: content.to_string(),
            state: PhantomData,
        }
    }

    pub fn publish(self) -> Post<Unmoderated> {
        Post {
            content: self.content,
            state: PhantomData,
        }
    }
}

impl Post<Unmoderated> {
    pub fn allow(self) -> Post<Published> {
        Post {
            content: self.content,
            state: PhantomData,
        }
    }

    pub fn deny(self) -> Post<Deleted> {
        Post {
            content: self.content,
            state: PhantomData,
        }
    }
}

impl Post<Published> {
    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn delete(self) -> Post<Deleted> {
        Post {
            content: self.content,
            state: PhantomData,
        }
    }
}

impl Post<Deleted> {
    pub fn content(&self) -> &str {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_publish_allow_delete() {
        let post = Post::new("Hello Rust!");
        let unmoderated = post.publish();
        let published = unmoderated.allow();
        assert_eq!(published.content(), "Hello Rust!");
        let deleted = published.delete();
        assert_eq!(deleted.content(), "Hello Rust!");
    }

    #[test]
    fn test_workflow_deny() {
        let post = Post::new("Spam");
        let unmoderated = post.publish();
        let deleted = unmoderated.deny();
        assert_eq!(deleted.content(), "Spam");
    }
}
