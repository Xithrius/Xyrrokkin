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
    /// Number of files that have become this type
    pub to: usize,
    /// Occurrences in which files used to be this type
    pub from: usize,
}

impl FileData {
    pub fn new(
        ext: Option<String>,
        amount: usize,
        new: usize,
        deleted: usize,
        to: usize,
        from: usize,
    ) -> Self {
        Self {
            ext,
            amount,
            new,
            deleted,
            to,
            from,
        }
    }
}

impl std::ops::AddAssign for FileData {
    fn add_assign(&mut self, other: Self) {
        *self = Self::new(
            other.ext,
            self.amount + other.amount,
            self.new + other.new,
            self.deleted + other.deleted,
            self.to + other.to,
            self.from + other.from
        );
    }
}
