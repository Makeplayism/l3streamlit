use leptos::*;
use leptos_router::*;
use crate::models::{GameState, StoryData, ChoiceType};
use crate::services::{StoryLoader, PathNavigator};
use crate::components::{StoryDisplay, ChoiceButtons, StoryTree, ControlPanel};

#[component]
pub fn App() -> impl IntoView {
    let (story_data, set_story_data) = create_signal(None::<StoryData>);
    let (game_state, set_game_state) = create_signal(GameState::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);

    // Load story data on component mount
    create_effect(move |_| {
        spawn_local(async move {
            match StoryLoader::load_default() {
                Ok(data) => {
                    set_story_data.set(Some(data));
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to load story data: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    });

    let make_choice = move |choice_type: ChoiceType| {
        set_game_state.update(|state| {
            state.add_choice(choice_type);
        });
    };

    let reset_game = move || {
        set_game_state.update(|state| {
            state.reset();
        });
    };

    view! {
        <Router>
            <div class="app-container">
                <header class="app-header">
                    <h1>"[L3]未来之门"</h1>
                    <p>"基于 Life 3.0 的交互式故事游戏"</p>
                </header>
                
                <main class="app-main">
                    {move || {
                        if loading.get() {
                            view! {
                                <div class="loading">
                                    <p>"加载故事数据中..."</p>
                                </div>
                            }.into_view()
                        } else if let Some(error_msg) = error.get() {
                            view! {
                                <div class="error">
                                    <p>"错误: " {error_msg}</p>
                                </div>
                            }.into_view()
                        } else if let Some(data) = story_data.get() {
                            let navigator = PathNavigator::new(&data);
                            view! {
                                <div class="game-container">
                                    <div class="game-content">
                                        <StoryDisplay 
                                            story_data=data.clone()
                                            game_state=game_state.get()
                                        />
                                        
                                        <ChoiceButtons 
                                            story_data=data.clone()
                                            game_state=game_state.get()
                                            on_choice=make_choice
                                        />
                                    </div>
                                    
                                    <aside class="game-sidebar">
                                        <StoryTree 
                                            story_data=data.clone()
                                            game_state=game_state.get()
                                        />
                                        
                                        <ControlPanel 
                                            game_state=game_state.get()
                                            on_reset=reset_game
                                        />
                                    </aside>
                                </div>
                            }.into_view()
                        } else {
                            view! {
                                <div class="empty">
                                    <p>"无法加载故事数据"</p>
                                </div>
                            }.into_view()
                        }
                    }}
                </main>
            </div>
        </Router>
    }
}