use leptos::*;
use crate::models::GameState;

#[component]
pub fn ControlPanel(
    game_state: GameState,
    on_reset: impl Fn() + 'static,
) -> impl IntoView {
    view! {
        <div class="control-panel">
            <h3 class="panel-title">"游戏控制"</h3>
            
            <div class="control-buttons">
                <button 
                    class="control-button reset-button"
                    on:click=move |_| on_reset()
                >
                    "🔄 重新开始"
                </button>
                
                <button 
                    class="control-button save-button"
                    disabled=move || game_state.get_path().is_empty()
                >
                    "💾 保存进度"
                </button>
                
                <button 
                    class="control-button load-button"
                >
                    "📁 加载进度"
                </button>
            </div>
            
            <div class="game-stats">
                <h4>"游戏统计"</h4>
                <div class="stats-grid">
                    <div class="stat-item">
                        <span class="stat-label">"已做选择:"</span>
                        <span class="stat-value">{game_state.get_level()}</span>
                    </div>
                    <div class="stat-item">
                        <span class="stat-label">"剩余选择:"</span>
                        <span class="stat-value">{6 - game_state.get_level()}</span>
                    </div>
                    <div class="stat-item">
                        <span class="stat-label">"路径长度:"</span>
                        <span class="stat-value">{game_state.get_path().len()}</span>
                    </div>
                </div>
            </div>
            
            {if game_state.is_complete() {
                view! {
                    <div class="completion-info">
                        <h4>"🎉 故事完成!"</h4>
                        <p>"你的最终路径: " <code>{game_state.get_path()}</code></p>
                        <p>"这是 126 种可能结局中的一种"</p>
                    </div>
                }.into_view()
            } else {
                view! {}.into_view()
            }}
        </div>
    }
}