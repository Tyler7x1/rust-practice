#[derive(Debug)]

pub struct Member {
    pub name: String,
    pub member_id: u32,
}

pub fn register(name: &str, member_id: u32) -> Member {
    Member {
        name: name.to_string(),
        member_id,
    }
}