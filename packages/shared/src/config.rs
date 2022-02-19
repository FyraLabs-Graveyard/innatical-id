// Functions relating to Config Files

use directories::{ProjectDirs};
use std::path::Path;

pub fn get_config_directory(cli_config: Option<&str>) -> Result<String, &'static str> {
    let config: String;

    if let Some(cli_config) = cli_config {
        config = String::from(Path::new(cli_config).as_os_str().to_os_string().into_string().unwrap())
    } else if let Some(proj_dirs) = ProjectDirs::from("com", "Innatical", "Innatical ID") {
        config = String::from(proj_dirs.config_dir().as_os_str().to_os_string().into_string().unwrap());
    } else {
        return Err("Config file not found");
    }
    
    
    Ok(config)
}