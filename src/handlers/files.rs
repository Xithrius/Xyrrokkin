#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FileData {
    /// The file extension name (if there is one)
    pub ext: Option<String>,
    /// The amount of times this file name has happened
    pub amount: usize,
    /// How many new files have this extension
    pub new: usize,
    /// How many times this type has been deleted
    pub deleted: usize,
}

impl FileData {
    pub fn new(ext: Option<String>, amount: usize, new: usize, deleted: usize) -> Self {
        Self {
            ext,
            amount,
            new,
            deleted,
        }
    }

    pub fn to_vec(&self) -> Vec<String> {
        vec![
            if let Some(e) = self.ext.clone() {
                e
            } else {
                "None".to_string()
            },
            self.amount.to_string(),
            self.new.to_string(),
            self.deleted.to_string(),
        ]
    }
}

impl std::ops::AddAssign for FileData {
    fn add_assign(&mut self, other: Self) {
        *self = Self::new(
            other.ext,
            self.amount + other.amount,
            self.new + other.new,
            self.deleted + other.deleted,
        );
    }
}
