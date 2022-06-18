#[cfg(test)]
mod tests {
    use crate::zip::open_zip;
    use std::{fmt::Error, fs, io::ErrorKind};
    use tempfile;

    #[test]
    fn open_zip_directory() {
        let mock_dir = tempfile::tempdir().unwrap();
        let mock_dir_path_name = mock_dir.path().to_str().unwrap();
        let tar_data_path = "./src/tests/data/TestZipFolder.zip";
        open_zip(tar_data_path, Some(mock_dir_path_name));
        println!("Here is the {}", mock_dir_path_name);

        let mut inside_mock_dir_name = mock_dir_path_name.to_string();
        inside_mock_dir_name.push_str("/TestZipFolder");

        // Test the directory was created and has all it's sub folders and files
        assert!(fs::metadata(mock_dir_path_name).unwrap().is_dir());
        assert_eq!(fs::read_dir(&inside_mock_dir_name).unwrap().count(), 3);

        // Test the nested folder is a directory and has it's sub files
        inside_mock_dir_name.push_str("/nestedFolder1");
        assert!(fs::metadata(&inside_mock_dir_name).unwrap().is_dir());
        assert_eq!(fs::read_dir(&inside_mock_dir_name).unwrap().count(), 1);
    }

    #[test]
    
    fn fails_cannot_find_directory() {
        let mock_dir = tempfile::tempdir().unwrap();
        let mock_dir_path_name = mock_dir.path().to_str().unwrap();
        let tar_data_path = "./src/tests/data/FolderNotFound.zip";
        let result = open_zip(tar_data_path, Some(mock_dir_path_name));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidInput);
    }
}
