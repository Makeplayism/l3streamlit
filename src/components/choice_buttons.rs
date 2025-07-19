use leptos::*;
use crate::models::{StoryData, GameState, ChoiceType};
use crate::services::PathNavigator;

#[component]
pub fn ChoiceButtons(
    story_data: StoryData,
    game_state: GameState,
    on_choice: impl Fn(ChoiceType) + 'static,
) -> impl IntoView {
    let navigator = PathNavigator::new(&story_data);
    let current_choice = navigator.get_current_choice(&game_state);
    
    view! {
        <div class="choice-buttons">
            {if let Some(choice) = current_choice {
                view! {
                    <div class="button-container">
                        <button 
                            class="choice-button red-button"
                            on:click=move |_| on_choice(ChoiceType::Red)
                        >
                            <span class="button-text">{choice.red.clone()}</span>
                        </button>
                        
                        <button 
                            class="choice-button blue-button"
                            on:click=move |_| on_choice(ChoiceType::Blue)
                        >
                            <span class="button-text">{choice.blue.clone()}</span>
                        </button>
                    </div>
                }.into_view()
            } else if game_state.is_complete() {
                view! {
                    <div class="completion-message">
                        <p>"故事已完成！"</p>
                        <p>"你的选择路径: " {game_state.get_path()}</p>
                    </div>
                }.into_view()
            } else {
                view! {
                    <div class="no-choices">
                        <p>"加载选择中..."</p>
                    </div>
                }.into_view()
            }}
        </div>
    }
}