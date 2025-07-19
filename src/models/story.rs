use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryData {
    #[serde(rename = "FM_CHOICE")]
    pub fm_choice: HashMap<String, ChoiceData>,
    #[serde(rename = "FM_STORY")]
    pub fm_story: HashMap<String, StoryContent>,
    #[serde(rename = "FM_START")]
    pub fm_start: StoryContent,
    #[serde(rename = "FM_NOEND")]
    pub fm_noend: StoryContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryContent {
    pub title: String,
    pub story: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceData {
    pub title: String,
    pub story: String,
    pub red: String,
    pub blue: String,
}

impl StoryData {
    pub fn get_story_by_path(&self, path: &str) -> Option<&StoryContent> {
        if path.is_empty() {
            Some(&self.fm_start)
        } else {
            self.fm_story.get(path)
        }
    }
    
    pub fn get_choice_by_level(&self, level: usize) -> Option<&ChoiceData> {
        self.fm_choice.get(&level.to_string())
    }
    
    pub fn get_final_story(&self) -> &StoryContent {
        &self.fm_noend
    }
}