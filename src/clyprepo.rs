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
  use std::fs::{read_dir, read_to_string, write};
  use std::path::Path;
  use tempfile::tempdir;

  #[test]
  fn save_clyp_file_is_created() {
    let dir = get_temp_dir();
    let path_str = dir
      .path()
      .to_str()
      .expect("could not convert tempdir path to str");

    let repo = ClypRepository::new(String::from(path_str));
    let name = "testclyp";
    let content = "test content";
    repo.save_clyp(name.to_string(), content.to_string());

    let file_path = Path::new(path_str).join(name);
    assert_eq!(file_path.exists(), true);

    let file_content = read_to_string(file_path).expect("could not read file");
    assert_eq!(file_content, content);
  }

  #[test]
  fn read_clyp_file_is_read() {
    let dir = get_temp_dir();
    let path_str = dir
      .path()
      .to_str()
      .expect("could not convert tempdir path to str");

    let name = "testclyp";
    let content = "test content";
    let file_path = Path::new(path_str).join(name);
    write(file_path, content).expect("could not write file");

    let repo = ClypRepository::new(String::from(path_str));
    let clyp_content = repo.read_clyp(name.to_string());

    assert_eq!(clyp_content, content);
  }

  #[test]
  fn clear_clyp_files_are_deleted() {
    let dir = get_temp_dir();
    let path_str = dir
      .path()
      .to_str()
      .expect("could not convert tempdir path to str");

    let names = ["testclyp1", "testclyp2", "testclyp3"];
    let content = "test content";

    for name in names.iter() {
      write(Path::new(path_str).join(name), content).expect("could not write file");
    }
    assert_eq!(read_dir(path_str).unwrap().count(), 3);

    let repo = ClypRepository::new(String::from(path_str));
    repo.clear_clyps();
    assert_eq!(read_dir(path_str).unwrap().count(), 0);
  }

  fn get_temp_dir() -> tempfile::TempDir {
    return tempdir().expect("could not create temporary directory");
  }
}
