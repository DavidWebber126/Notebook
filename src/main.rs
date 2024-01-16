use axum::{
    routing::{get, post},
    response::{Redirect, Html},
    Router
};

use std::fs;

// Check if person is logged in
// If they are then redirect to note/homepage
// Otherwise redirect to login page
async fn is_logged_in() -> Redirect {
    Redirect::to("/notes")
}

// Create a new note for the user
async fn new_note() -> Html<String> {
    let notepad = fs::read_to_string("note_page.html").unwrap();
    Html(notepad)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(is_logged_in))
        .route("/notes", get(new_note));

    println!("Running on https://localhost:3000");
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
