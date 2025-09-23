use std::path::Path;

pub async fn get_directory_files(path: String) -> Result<Vec<String>, String> {
    let path = Path::new(&path);
    let parent_dir = match path.parent() {
        Some(p) => p,
        None => return Err("Could not determine parent directory".to_string()),
    };

    let mut image_files = Vec::new();
    let image_extensions = ["png", "jpg", "jpeg", "webp"];

    for entry in std::fs::read_dir(parent_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if image_extensions.contains(&ext) {
                image_files.push(path.to_str().unwrap().to_string());
            }
        }
    }

    image_files.sort();

    Ok(image_files)
}
