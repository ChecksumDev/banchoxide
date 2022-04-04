pub struct Achievement {
    pub id: i32,
    pub file: String,
    pub name: String,
    pub desc: String,
    pub cond: fn(i32) -> bool,
}

impl Achievement {
    pub fn new(id: i32, file: String, name: String, desc: String, cond: fn(i32) -> bool) -> Self {
        Achievement {
            id,
            file,
            name,
            desc,
            cond,
        }
    }
}

impl std::fmt::Display for Achievement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}+{}+{}", self.file, self.name, self.desc)
    }
}