use crate::models::{GameState, StoryData, StoryContent, ChoiceData};

pub struct PathNavigator<'a> {
    story_data: &'a StoryData,
}

impl<'a> PathNavigator<'a> {
    pub fn new(story_data: &'a StoryData) -> Self {
        Self { story_data }
    }
    
    pub fn get_current_story(&self, game_state: &GameState) -> Option<&StoryContent> {
        let path = game_state.get_path();
        self.story_data.get_story_by_path(path)
    }
    
    pub fn get_current_choice(&self, game_state: &GameState) -> Option<&ChoiceData> {
        if game_state.can_make_choice() {
            self.story_data.get_choice_by_level(game_state.get_level())
        } else {
            None
        }
    }
    
    pub fn get_final_story(&self) -> &StoryContent {
        self.story_data.get_final_story()
    }
    
    pub fn generate_tree_visualization(&self, game_state: &GameState) -> String {
        let mut tree = String::new();
        tree.push_str("故事路径树:\n");
        
        for level in 0..=game_state.get_level() {
            let prefix = "  ".repeat(level);
            if level < game_state.get_level() {
                let choice_char = game_state.get_path().chars().nth(level).unwrap_or('?');
                let choice_name = match choice_char {
                    'R' => "红色",
                    'B' => "蓝色",
                    _ => "未知",
                };
                tree.push_str(&format!("{}├─ Level {}: {}\n", prefix, level + 1, choice_name));
            } else if level == game_state.get_level() && game_state.can_make_choice() {
                tree.push_str(&format!("{}├─ Level {}: [当前选择]\n", prefix, level + 1));
            }
        }
        
        if game_state.is_complete() {
            tree.push_str(&format!("{}└─ 完整路径: {}\n", 
                "  ".repeat(game_state.get_level()), 
                game_state.get_path()));
        }
        
        tree
    }
}