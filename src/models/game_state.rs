use crate::models::{Choice, ChoiceType};

#[derive(Debug, Clone, Default)]
pub struct GameState {
    pub choice_path: String,
    pub current_level: usize,
    pub choices: Vec<Choice>,
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn add_choice(&mut self, choice_type: ChoiceType) {
        let choice = Choice::new(choice_type.clone(), self.current_level);
        self.choices.push(choice);
        self.choice_path.push(choice_type.as_char());
        self.current_level += 1;
    }
    
    pub fn reset(&mut self) {
        self.choice_path.clear();
        self.current_level = 0;
        self.choices.clear();
    }
    
    pub fn is_complete(&self) -> bool {
        self.current_level >= 6
    }
    
    pub fn get_path(&self) -> &str {
        &self.choice_path
    }
    
    pub fn get_level(&self) -> usize {
        self.current_level
    }
    
    pub fn can_make_choice(&self) -> bool {
        self.current_level < 6
    }
}