#[derive(Debug)]
pub struct Context {
    cwd: std::path::PathBuf
}

impl Default for Context {
    fn default() -> Self {
        Self { cwd: std::path::PathBuf::from("~/") }
    }
}


