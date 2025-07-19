use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use axum::{
    routing::get,
    Router,
};
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use std::env;

mod models;
mod components;
mod services;
mod utils;

#[cfg(test)]
mod tests;

use components::App;

#[tokio::main]
async fn main() {
    console_error_panic_hook::set_once();
    
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|| view! { <App/> }).await;

    // Build the Axum router
    let app = Router::new()
        .route("/api/*fn_name", get(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, || view! { <App/> })
        .fallback(leptos_axum::file_and_error_handler(leptos_axum::handle_server_fns))
        .layer(
            ServiceBuilder::new()
                .layer(tower_http::services::ServeDir::new(&leptos_options.site_root))
        );

    // Run the server
    println!("🚀 L3 Story Game server starting at http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");
    
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // This is required when compiling for client-side
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> });
}