pub struct ClypRepository {
  clyps_dir: String,
}
impl ClypRepository {
  pub fn new(clyps_dir: String) -> ClypRepository {
    ClypRepository {
      clyps_dir: clyps_dir,
    }
  }

  pub fn save_clyp(&self, name: String, content: String) {
    let path = self.get_path_for_name(name);
    std::fs::write(path, content).expect("Unable to write file");
  }

  pub fn read_clyp(&self, name: String) -> String {
    let path = self.get_path_for_name(name);
    let content =
      std::fs::read_to_string(&path).expect(&format!("could not read file `{}`", path.display()));
    return content;
  }

  fn get_path_for_name(&self, name: String) -> std::path::PathBuf {
    return [&self.clyps_dir, &name].iter().collect();
  }
}
