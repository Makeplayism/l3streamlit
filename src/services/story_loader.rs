use crate::models::StoryData;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoryLoaderError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("Story file not found: {0}")]
    NotFound(String),
}

pub struct StoryLoader;

impl StoryLoader {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<StoryData, StoryLoaderError> {
        let path = path.as_ref();
        
        if !path.exists() {
            return Err(StoryLoaderError::NotFound(path.display().to_string()));
        }
        
        let content = std::fs::read_to_string(path)?;
        let story_data: StoryData = toml::from_str(&content)?;
        
        Ok(story_data)
    }
    
    pub fn load_default() -> Result<StoryData, StoryLoaderError> {
        let default_path = "../docs/FM_STORY.toml";
        Self::load_from_file(default_path)
    }
}