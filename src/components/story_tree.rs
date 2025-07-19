use leptos::*;
use crate::models::{StoryData, GameState};
use crate::services::PathNavigator;

#[component]
pub fn StoryTree(
    story_data: StoryData,
    game_state: GameState,
) -> impl IntoView {
    let navigator = PathNavigator::new(&story_data);
    let tree_text = navigator.generate_tree_visualization(&game_state);
    
    view! {
        <div class="story-tree">
            <h3 class="tree-title">"故事路径"</h3>
            <div class="tree-content">
                <pre class="tree-text">{tree_text}</pre>
            </div>
            
            <div class="path-info">
                <p><strong>"当前层级: "</strong> {game_state.get_level() + 1} "/6"</p>
                <p><strong>"选择路径: "</strong> {game_state.get_path()}</p>
                <p><strong>"进度: "</strong> {format!("{:.1}%", (game_state.get_level() as f32 / 6.0) * 100.0)}</p>
            </div>
        </div>
    }
}