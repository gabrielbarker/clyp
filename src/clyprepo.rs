use std::fs::{create_dir, read_dir, read_to_string, remove_dir_all, write};

pub struct ClypRepository {
  clyps_dir: std::path::PathBuf,
}

impl ClypRepository {
  pub fn new(clyps_dir: std::path::PathBuf) -> ClypRepository {
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

  pub fn list_clyps(&self) {
    for file in read_dir(&self.clyps_dir).unwrap() {
      let file_name = file
        .unwrap()
        .path()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
      println!("{}", file_name);
    }
  }

  pub fn get_most_recent_clyp(&self) -> String {
    let dir = read_dir(&self.clyps_dir);
    let mut time = std::time::SystemTime::UNIX_EPOCH;
    let mut path = std::path::PathBuf::new();
    for file in dir.unwrap() {
      let file_path = file.unwrap().path();
      let metadata = std::fs::metadata(file_path.clone()).expect("");
      if metadata.modified().unwrap() > time {
        time = metadata.modified().unwrap();
        path = file_path;
      }
    }
    return read_to_string(&path).expect(&format!("could not read file `{}`", path.display()));
  }

  fn get_path_for_name(&self, name: String) -> std::path::PathBuf {
    return self.clyps_dir.join(name.as_str());
  }
}

#[cfg(test)]
mod clyprepo_tests {
  use super::ClypRepository;
  use std::fs::{read_to_string, write};
  use tempfile::tempdir;

  #[test]
  fn save_clyp_file_is_created() {
    let dir = get_temp_dir();
    let path = dir.path();

    let repo = ClypRepository::new(path.to_path_buf());
    let name = "testclyp";
    let content = "test content";
    repo.save_clyp(name.to_string(), content.to_string());

    let file_path = path.join(name);
    assert_eq!(file_path.exists(), true);

    let file_content = read_to_string(file_path).expect("could not read file");
    assert_eq!(file_content, content);
  }

  #[test]
  fn read_clyp_file_is_read() {
    let dir = get_temp_dir();
    let path = dir.path();

    let name = "testclyp";
    let content = "test content";
    let file_path = path.join(name);
    write(file_path, content).expect("could not write file");

    let repo = ClypRepository::new(path.to_path_buf());
    let clyp_content = repo.read_clyp(name.to_string());

    assert_eq!(clyp_content, content);
  }

  #[test]
  fn clear_clyp_files_are_deleted() {
    let dir = get_temp_dir();
    let path = dir.path();

    let names = ["testclyp1", "testclyp2", "testclyp3"];
    let content = "test content";

    for name in names.iter() {
      write(path.join(name), content).expect("could not write file");
    }
    assert_eq!(path.read_dir().unwrap().count(), 3);

    let repo = ClypRepository::new(path.to_path_buf());
    repo.clear_clyps();
    assert_eq!(path.read_dir().unwrap().count(), 0);
  }

  #[test]
  fn get_most_recent_clyp_most_recent_clyp() {
    let dir = get_temp_dir();
    let path = dir.path();

    let names = ["testclyp1", "testclyp2", "testclyp3"];
    let content = "test content";

    for name in names.iter() {
      write(path.join(name), content).expect("could not write file");
    }

    std::thread::sleep(std::time::Duration::from_secs(1));

    let latest_name = "testclyp_latest";
    let latest_content = "test content latest";
    write(path.join(latest_name), latest_content).expect("could not write file");

    assert_eq!(path.read_dir().unwrap().count(), 4);

    let repo = ClypRepository::new(path.to_path_buf());
    let clyp = repo.get_most_recent_clyp();

    assert_eq!(clyp, latest_content);
  }

  fn get_temp_dir() -> tempfile::TempDir {
    return tempdir().expect("could not create temporary directory");
  }
}
