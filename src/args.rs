#[derive(Debug, PartialEq)]
pub struct Arg {
    standalone: bool,
    name: String,
}

impl PartialEq<String> for Arg {
    fn eq(&self, other: &String) -> bool {
        &self.name == other
    }
}

impl PartialEq<str> for Arg {
    fn eq(&self, other: &str) -> bool {
        &self.name == other
    }
}

impl Arg {
    pub fn new<N>(name: N, standalone: bool) -> Self
    where
        N: Into<String>,
    {
        Self {
            name: name.into(),
            standalone,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn is_standalone(&self) -> bool {
        self.standalone
    }
}
