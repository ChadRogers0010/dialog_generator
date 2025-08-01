#![allow(unused)]
use std::path::PathBuf;

pub struct Builder {
    pub path: String,
    pub module_handles: Vec<PathBuf>,
    pub header: String,
}
pub fn build_query(path: impl Into<String> + Clone, lines_per_module: usize) {
    let csv_file = std::fs::read_to_string(path.clone().into()).unwrap();
    let mut lines = csv_file.lines();
    let header = lines.next().unwrap().to_string();

    let mut builder = Builder {
        path: path.into(),
        module_handles: Vec::new(),
        header,
    };
}
