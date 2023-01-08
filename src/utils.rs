use std::fs::read_to_string;

pub fn read_src(filename: &str) -> String {
  read_to_string(filename).expect(format!("Error: Filepath \"{}\" does not exist.", filename).as_str())
}
