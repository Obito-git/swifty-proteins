const LOCAL_STORAGE_DEFAULT_PATH: &str = "database/local_storage";

//TODO: error handling
pub fn read(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
