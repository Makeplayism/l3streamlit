#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::services::*;
    
    #[test]
    fn test_story_data_loading() {
        // Test loading story data from TOML
        let story_data = StoryLoader::load_default();
        assert!(story_data.is_ok(), "Should load story data successfully");
        
        let data = story_data.unwrap();
        assert!(!data.fm_choice.is_empty(), "Should have choice data");
        assert!(!data.fm_story.is_empty(), "Should have story data");
    }
    
    #[test]
    fn test_story_navigation() {
        // Mock story data for testing
        let mut story_data = StoryData {
            fm_choice: std::collections::HashMap::new(),
            fm_story: std::collections::HashMap::new(),
            fm_start: StoryContent {
                title: "开始".to_string(),
                story: "故事开始".to_string(),
            },
            fm_noend: StoryContent {
                title: "结束".to_string(),
                story: "故事结束".to_string(),
            },
        };
        
        // Add test story
        story_data.fm_story.insert("R".to_string(), StoryContent {
            title: "红色路径".to_string(),
            story: "选择了红色".to_string(),
        });
        
        let navigator = PathNavigator::new(&story_data);
        let mut game_state = GameState::new();
        
        // Test initial state
        let initial_story = navigator.get_current_story(&game_state);
        assert!(initial_story.is_some());
        assert_eq!(initial_story.unwrap().title, "开始");
        
        // Test after making a choice
        game_state.add_choice(ChoiceType::Red);
        let after_choice = navigator.get_current_story(&game_state);
        assert!(after_choice.is_some());
        assert_eq!(after_choice.unwrap().title, "红色路径");
    }
    
    #[test]
    fn test_choice_validation() {
        let red_choice = ChoiceType::Red;
        let blue_choice = ChoiceType::Blue;
        
        assert_eq!(red_choice.as_char(), 'R');
        assert_eq!(blue_choice.as_char(), 'B');
        
        assert_eq!(ChoiceType::from_char('R'), Some(ChoiceType::Red));
        assert_eq!(ChoiceType::from_char('B'), Some(ChoiceType::Blue));
        assert_eq!(ChoiceType::from_char('X'), None);
    }
}