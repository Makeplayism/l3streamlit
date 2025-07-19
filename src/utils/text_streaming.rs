use std::time::Duration;

pub struct TextStreamer {
    pub delay_ms: u64,
}

impl TextStreamer {
    pub fn new(delay_ms: u64) -> Self {
        Self { delay_ms }
    }
    
    pub fn default() -> Self {
        Self::new(50)
    }
    
    pub fn smart_chunks(&self, text: &str) -> Vec<String> {
        // Split text into smart chunks for streaming
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        
        for char in text.chars() {
            current_chunk.push(char);
            
            // Break at natural pause points
            if matches!(char, '。' | '！' | '？' | '；' | '：' | '，' | '\n') {
                chunks.push(current_chunk.clone());
                current_chunk.clear();
            }
        }
        
        // Add any remaining text
        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }
        
        chunks
    }
    
    pub fn get_delay(&self) -> Duration {
        Duration::from_millis(self.delay_ms)
    }
}