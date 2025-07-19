use leptos::*;
use crate::models::{StoryData, GameState};
use crate::services::PathNavigator;

#[component]
pub fn StoryDisplay(
    story_data: StoryData,
    game_state: GameState,
) -> impl IntoView {
    let navigator = PathNavigator::new(&story_data);
    
    let current_story = navigator.get_current_story(&game_state);
    let current_choice = navigator.get_current_choice(&game_state);
    
    view! {
        <div class="story-display">
            {if let Some(story) = current_story {
                view! {
                    <div class="story-content">
                        <h2 class="story-title">{story.title.clone()}</h2>
                        <div class="story-text">
                            <StreamingText text=story.story.clone() />
                        </div>
                    </div>
                }.into_view()
            } else {
                view! {
                    <div class="story-content">
                        <h2 class="story-title">"故事加载中..."</h2>
                    </div>
                }.into_view()
            }}
            
            {if let Some(choice) = current_choice {
                view! {
                    <div class="choice-prompt">
                        <h3 class="choice-title">{choice.title.clone()}</h3>
                        <p class="choice-story">{choice.story.clone()}</p>
                    </div>
                }.into_view()
            } else if game_state.is_complete() {
                let final_story = navigator.get_final_story();
                view! {
                    <div class="final-story">
                        <h2 class="final-title">{final_story.title.clone()}</h2>
                        <div class="final-text">
                            <StreamingText text=final_story.story.clone() />
                        </div>
                    </div>
                }.into_view()
            } else {
                view! {
                    <div class="empty-choice">
                        <p>"准备开始新的故事..."</p>
                    </div>
                }.into_view()
            }}
        </div>
    }
}

#[component]
pub fn StreamingText(text: String) -> impl IntoView {
    let (displayed_text, set_displayed_text) = create_signal(String::new());
    let (is_streaming, set_is_streaming) = create_signal(true);
    
    create_effect(move |_| {
        let text_clone = text.clone();
        spawn_local(async move {
            set_displayed_text.set(String::new());
            set_is_streaming.set(true);
            
            let chars: Vec<char> = text_clone.chars().collect();
            for (i, _) in chars.iter().enumerate() {
                let partial_text: String = chars[0..=i].iter().collect();
                set_displayed_text.set(partial_text);
                
                // Sleep for streaming effect
                gloo_timers::future::TimeoutFuture::new(50).await;
            }
            
            set_is_streaming.set(false);
        });
    });
    
    view! {
        <div class="streaming-text">
            <p>{move || displayed_text.get()}</p>
            {move || {
                if is_streaming.get() {
                    view! {
                        <span class="cursor">"|"</span>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }}
        </div>
    }
}