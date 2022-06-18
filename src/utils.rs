use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub fn get_input_output_path(
    input_path: &str,
    output_path: Option<&str>,
) -> Result<(PathBuf, PathBuf), Error> {
    let input_file_path = PathBuf::from(input_path);

    if !input_file_path.exists() {
        let not_a_file_error = Error::new(
            ErrorKind::InvalidInput,
            format!("Target path is not a directory!: {}", input_file_path.display()),
        );
        return Err(not_a_file_error);
    }
    let output_file_path = match output_path {
        Some(output_path) => output_path,
        _ => ".",
    };
    let input_path_buf = PathBuf::from(input_path);
    let output_path_buf = PathBuf::from(output_file_path);

    Ok((input_path_buf, output_path_buf))
}
