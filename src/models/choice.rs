#[derive(Debug, Clone, PartialEq)]
pub enum ChoiceType {
    Red,
    Blue,
}

impl ChoiceType {
    pub fn as_char(&self) -> char {
        match self {
            ChoiceType::Red => 'R',
            ChoiceType::Blue => 'B',
        }
    }
    
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'R' => Some(ChoiceType::Red),
            'B' => Some(ChoiceType::Blue),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Choice {
    pub choice_type: ChoiceType,
    pub level: usize,
}

impl Choice {
    pub fn new(choice_type: ChoiceType, level: usize) -> Self {
        Self { choice_type, level }
    }
}