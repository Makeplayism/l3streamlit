#[cfg(test)]
mod tests {
    use crate::models::*;
    
    #[test]
    fn test_game_state_initialization() {
        let game_state = GameState::new();
        
        assert_eq!(game_state.get_level(), 0);
        assert_eq!(game_state.get_path(), "");
        assert!(game_state.choices.is_empty());
        assert!(game_state.can_make_choice());
        assert!(!game_state.is_complete());
    }
    
    #[test]
    fn test_game_state_choices() {
        let mut game_state = GameState::new();
        
        // Add some choices
        game_state.add_choice(ChoiceType::Red);
        assert_eq!(game_state.get_level(), 1);
        assert_eq!(game_state.get_path(), "R");
        
        game_state.add_choice(ChoiceType::Blue);
        assert_eq!(game_state.get_level(), 2);
        assert_eq!(game_state.get_path(), "RB");
        
        game_state.add_choice(ChoiceType::Red);
        game_state.add_choice(ChoiceType::Red);
        game_state.add_choice(ChoiceType::Blue);
        game_state.add_choice(ChoiceType::Blue);
        
        assert_eq!(game_state.get_level(), 6);
        assert_eq!(game_state.get_path(), "RBRRBB");
        assert!(!game_state.can_make_choice());
        assert!(game_state.is_complete());
    }
    
    #[test]
    fn test_game_state_reset() {
        let mut game_state = GameState::new();
        
        // Add choices
        game_state.add_choice(ChoiceType::Red);
        game_state.add_choice(ChoiceType::Blue);
        
        // Reset
        game_state.reset();
        
        assert_eq!(game_state.get_level(), 0);
        assert_eq!(game_state.get_path(), "");
        assert!(game_state.choices.is_empty());
        assert!(game_state.can_make_choice());
        assert!(!game_state.is_complete());
    }
    
    #[test]
    fn test_choice_path_building() {
        let mut game_state = GameState::new();
        
        let test_choices = vec![
            ChoiceType::Red,
            ChoiceType::Blue,
            ChoiceType::Red,
            ChoiceType::Blue,
            ChoiceType::Red,
            ChoiceType::Blue,
        ];
        
        for choice in test_choices {
            game_state.add_choice(choice);
        }
        
        assert_eq!(game_state.get_path(), "RBRBR");
        assert_eq!(game_state.choices.len(), 6);
        assert!(game_state.is_complete());
    }
}