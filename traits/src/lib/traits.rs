pub trait Describable {
    fn describe(&self) -> String;
}

pub struct Book {
    pub title: String,
    pub author: String,
}

impl Describable for Book {
    fn describe(&self) -> String {
        format!("{}, by {}", &self.title, &self.author)
    }
}

pub fn notify(msg: &impl Describable) -> String {
    format!("1 message from {}", msg.describe())
}