use std::{env, fmt::format, path::Path};
use sys_info;



#[derive(Debug)]
pub struct Config {
    pub user: String,
    pub os: String,
    pub download_dir: String,
}

impl Default for Config {
    fn default() -> Self {
       let user = match env::var("USER").or_else(|_| env::var("USERNAME")) {
            Ok(user) =>user,
            Err(e) => {
                println!("failed to get user because {}", e);
                "".to_string()
            },
        };

       let os = match sys_info::os_type() {
           Ok(os) => os,
           Err(e) => {
               println!("failed to get operating system because {}", e);
               "".to_string()
           },
       };

       let home_dir = env::var("HOME").unwrap_or_else(|_| env::var("USERPROFILE").unwrap());
       let download_dir = Path::new(&home_dir).join("Downloads").join("Yad").to_str().unwrap_or("_").to_string();

        Config {
            user: user.to_string(),
            os: os.to_string(),
            download_dir,
        }
    }
}
