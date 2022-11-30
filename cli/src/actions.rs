pub enum Actions {
    Deformat
}

impl Actions {
    pub fn from_str(s: &str) -> Actions {
        let action = match s {
            "deformat" => Actions::Deformat,
            _ => unimplemented!()
        };

        action
    }
}
