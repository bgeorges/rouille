/*
*
*
*/

pub struct User {
    pub id: i64,
    pub name: String,
    pub measurements_count: Option<i64>,
}

impl User {
    pub fn new(id: i64, name: String) -> Self {
        User {
            id,
            name,
            measurements_count: None,
        }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
