use std::fs::{create_dir, read_to_string, remove_dir_all, write};

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
    write(path, content).expect("unable to write file");
  }

  pub fn read_clyp(&self, name: String) -> String {
    let path = self.get_path_for_name(name);
    let content =
      read_to_string(&path).expect(&format!("could not read file `{}`", path.display()));
    return content;
  }

  pub fn clear_clyps(&self) {
    remove_dir_all(&self.clyps_dir).expect("could not delete clyps");
    create_dir(&self.clyps_dir).expect("could not create clyps directory");
  }

  fn get_path_for_name(&self, name: String) -> std::path::PathBuf {
    return [&self.clyps_dir, &name].iter().collect();
  }
}

#[cfg(test)]
mod clyprepo_tests {
  use super::ClypRepository;
  use std::fs::{read_to_string, write};
  use tempfile::tempdir;

  #[test]
  fn save_clyp_file_is_created() {
    let dir = tempdir().expect("could not create temporary directory");
    let path_str = dir
      .path()
      .to_str()
      .expect("could not convert tempdir path to str");

    let repo = ClypRepository::new(String::from(path_str));
    let name = "testclyp";
    let content = "test content";
    repo.save_clyp(name.to_string(), content.to_string());

    let file_path = std::path::Path::new(path_str).join(name);
    assert_eq!(file_path.exists(), true);

    let file_content = read_to_string(file_path).expect("could not read file");
    assert_eq!(file_content, content);
  }

  #[test]
  fn read_clyp_file_is_read() {
    let dir = tempdir().expect("could not create temporary directory");
    let path_str = dir
      .path()
      .to_str()
      .expect("could not convert tempdir path to str");

    let name = "testclyp";
    let content = "test content";
    let file_path = std::path::Path::new(path_str).join(name);
    write(file_path, content).expect("could not write file");

    let repo = ClypRepository::new(String::from(path_str));
    let clyp_content = repo.read_clyp(name.to_string());

    assert_eq!(clyp_content, content);
  }
}
