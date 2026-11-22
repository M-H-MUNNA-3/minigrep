pub struct Config<'a> {
    pub query: &'a str,
    pub path: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(path: &'a str, query: &'a str) -> Self {
        Self { query, path }
    }
}
