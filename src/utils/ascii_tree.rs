use crate::models::GameState;

pub struct AsciiTreeGenerator;

impl AsciiTreeGenerator {
    pub fn generate_tree(game_state: &GameState) -> String {
        let mut tree = String::new();
        let path = game_state.get_path();
        
        tree.push_str("故事路径树:\n");
        tree.push_str("════════════\n");
        
        // Root node
        tree.push_str("📚 开始\n");
        
        // Generate tree for each level
        for level in 0..6 {
            let indent = "  ".repeat(level + 1);
            
            if level < path.len() {
                let choice_char = path.chars().nth(level).unwrap();
                let (icon, name) = match choice_char {
                    'R' => ("🔴", "红色"),
                    'B' => ("🔵", "蓝色"),
                    _ => ("❓", "未知"),
                };
                
                tree.push_str(&format!("{}├─ {} {} (Level {})\n", 
                    indent, icon, name, level + 1));
            } else if level == path.len() && game_state.can_make_choice() {
                tree.push_str(&format!("{}├─ ❓ [当前选择] (Level {})\n", 
                    indent, level + 1));
                break;
            } else {
                tree.push_str(&format!("{}├─ ⚪ [未选择] (Level {})\n", 
                    indent, level + 1));
            }
        }
        
        // Add completion indicator
        if game_state.is_complete() {
            tree.push_str("  └─ 🎯 故事完成!\n");
            tree.push_str(&format!("     完整路径: {}\n", path));
        }
        
        tree.push_str("════════════\n");
        tree
    }
    
    pub fn generate_compact_tree(game_state: &GameState) -> String {
        let path = game_state.get_path();
        let mut tree = String::new();
        
        tree.push_str("Path: ");
        if path.is_empty() {
            tree.push_str("📚");
        } else {
            tree.push_str("📚");
            for c in path.chars() {
                match c {
                    'R' => tree.push_str(" → 🔴"),
                    'B' => tree.push_str(" → 🔵"),
                    _ => tree.push_str(" → ❓"),
                }
            }
        }
        
        if game_state.can_make_choice() {
            tree.push_str(" → ❓");
        }
        
        tree
    }
    
    pub fn generate_statistics(game_state: &GameState) -> String {
        let mut stats = String::new();
        
        stats.push_str("游戏统计:\n");
        stats.push_str("────────────\n");
        stats.push_str(&format!("当前层级: {}/6\n", game_state.get_level()));
        stats.push_str(&format!("选择路径: {}\n", 
            if game_state.get_path().is_empty() { 
                "无" 
            } else { 
                game_state.get_path() 
            }));
        stats.push_str(&format!("进度: {:.1}%\n", 
            (game_state.get_level() as f32 / 6.0) * 100.0));
        
        let red_count = game_state.get_path().chars().filter(|&c| c == 'R').count();
        let blue_count = game_state.get_path().chars().filter(|&c| c == 'B').count();
        
        stats.push_str(&format!("红色选择: {}\n", red_count));
        stats.push_str(&format!("蓝色选择: {}\n", blue_count));
        
        if game_state.is_complete() {
            stats.push_str("状态: 完成 ✅\n");
        } else {
            stats.push_str("状态: 进行中 ⏳\n");
        }
        
        stats.push_str("────────────\n");
        stats
    }
}