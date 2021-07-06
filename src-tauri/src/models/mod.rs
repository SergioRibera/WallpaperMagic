use std::fmt;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub enum RandomizeType {
    NextAsRandom,
    ShuffleRandom,
    Disabled
}

impl std::str::FromStr for RandomizeType {
    type Err = ::strum::ParseError;

    fn from_str(s: &str) -> Result<RandomizeType, Self::Err> {
        match s {
            s if s.eq_ignore_ascii_case("NextAsRandom") => Ok(RandomizeType::NextAsRandom),
            s if s.eq_ignore_ascii_case("ShuffleRandom") => Ok(RandomizeType::ShuffleRandom),
            s if s.eq_ignore_ascii_case("Disabled") => Ok(RandomizeType::Disabled),
            _ => ::std::result::Result::Err(::strum::ParseError::VariantNotFound),
        }
    }
}
impl fmt::Display for RandomizeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum BackgroundType {
    Mix,
    OnlyImg,
    OnlyAnimated
}

impl std::str::FromStr for BackgroundType {
    type Err = ::strum::ParseError;

    fn from_str(s: &str) -> Result<BackgroundType, Self::Err> {
        match s {
            s if s.eq_ignore_ascii_case("Mix") => Ok(BackgroundType::Mix),
            s if s.eq_ignore_ascii_case("OnlyImg") => Ok(BackgroundType::OnlyImg),
            s if s.eq_ignore_ascii_case("OnlyAnimated") => Ok(BackgroundType::OnlyAnimated),
            _ => ::std::result::Result::Err(::strum::ParseError::VariantNotFound),
        }
    }
}
impl fmt::Display for BackgroundType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// #[derive(strum_macros::ToString, Debug)]
// enum BackgroundCrop {
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationFiles {
    pub directories: Vec<String>,
    pub files: Vec<String>
}

impl ApplicationFiles {
    /// Set the application files's directories.
    pub fn set_directories(&mut self, directories: Vec<String>) {
        self.directories = directories;
    }

    /// Set the application files's files.
    pub fn set_files(&mut self, files: Vec<String>) {
        self.files = files;
    }

    /// Get a mutable reference to the application files's directories.
    pub fn directories_mut(&mut self) -> &mut Vec<String> {
        &mut self.directories
    }

    /// Get a mutable reference to the application files's files.
    pub fn files_mut(&mut self) -> &mut Vec<String> {
        &mut self.files
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationDataConfig {
    port: u16,
    wallpapers_file: ApplicationFiles,
    randomize: String,
    bg_type: String,
    enable_sound: bool
}

impl ApplicationDataConfig {
    /// Get a reference to the application data config's port.
    pub fn port(&self) -> &u16 {
        &self.port
    }

    /// Get a reference to the application data config's wallpapers file.
    pub fn wallpapers_file(&self) -> &ApplicationFiles {
        &self.wallpapers_file
    }

    /// Get a reference to the application data config's randomize.
    pub fn randomize(&self) -> &str {
        self.randomize.as_str()
    }

    /// Get a reference to the application data config's bg type.
    pub fn bg_type(&self) -> &str {
        self.bg_type.as_str()
    }

    /// Get a reference to the application data config's enable sound.
    pub fn enable_sound(&self) -> &bool {
        &self.enable_sound
    }

    /// Set the application data config's port.
    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    /// Set the application data config's randomize.
    pub fn set_randomize(&mut self, randomize: String) {
        self.randomize = randomize;
    }

    /// Set the application data config's bg type.
    pub fn set_bg_type(&mut self, bg_type: String) {
        self.bg_type = bg_type;
    }

    /// Set the application data config's enable sound.
    pub fn set_enable_sound(&mut self, enable_sound: bool) {
        self.enable_sound = enable_sound;
    }

    /// Get a mutable reference to the application data config's wallpapers file.
    pub fn wallpapers_file_mut(&mut self) -> &mut ApplicationFiles {
        &mut self.wallpapers_file
    }

    /// Set the application data config's wallpapers file.
    pub fn set_wallpapers_file(&mut self, wallpapers_file: ApplicationFiles) {
        self.wallpapers_file = wallpapers_file;
    }
}

impl Default for ApplicationFiles {
    fn default() -> Self {
        ApplicationFiles {
            directories: vec![],
            files: vec![]
        }
    }
}

impl Default for ApplicationDataConfig {
    fn default() -> Self {
        ApplicationDataConfig {
            port: 6789,
            wallpapers_file: ApplicationFiles::default(),
            randomize: RandomizeType::ShuffleRandom.to_string(),
            bg_type: BackgroundType::Mix.to_string(),
            enable_sound: false
        }
    }
}
