#[cfg(test)]
mod tests {
    use crate::open_tar;
    use std::{fs, io::ErrorKind};
    use tempfile;

    #[test]
    fn open_tar_directory() {
        // let mock_dir_name = "mock_dir";
        let mock_dir = tempfile::tempdir().unwrap();
        let mock_dir_path_name = mock_dir.path().to_str().unwrap();
        let tar_data_path = "./src/tests/data/External_test_data.tar.gz";

        // WHEN
        open_tar(tar_data_path, Some(mock_dir_path_name));
        println!("Here is the {}", mock_dir_path_name);

        let mut inside_mock_dir_name = mock_dir_path_name.to_string();
        inside_mock_dir_name.push_str("/External_test_data");

        assert!(fs::metadata(mock_dir_path_name).unwrap().is_dir());
        assert_eq!(fs::read_dir(&inside_mock_dir_name).unwrap().count(), 14);
    }

    #[test]
    fn fails_cannot_find_directory() {
        let mock_dir = tempfile::tempdir().unwrap();
        let mock_dir_path_name = mock_dir.path().to_str().unwrap();
        let tar_data_path = "./src/tests/data/FolderNotFound.tar.gz";
        let result = open_tar(tar_data_path, Some(mock_dir_path_name));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidInput);
    }
}
