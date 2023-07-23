use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use axum_macros;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Pool, Postgres};
use std::{net::SocketAddr, sync::Arc};

type Conn = Arc<Pool<Postgres>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // DB connection
    let url = "postgres://lms:passwd@localhost:5432/lms";

    // let pool = sqlx::postgres::PgPool::connect(url).await?;
    let pool = PgPool::connect(url).await.unwrap();
    tracing::info!("connection to DB is established!");

    // State
    let state_pool = Arc::new(pool);

    // Migrate Tables
    sqlx::migrate!("./migrations").run(&*state_pool).await?;

    // Router
    let app = Router::new()
        .route("/", get(root))
        .route("/books", post(create_book))
        .route("/books", get(get_books))
        .with_state(state_pool);

    // Exposing to port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!!!"
}

// TODO
// 1. Create book
// 2. Get books
// 3. Delete boook

async fn create_book(State(pool): State<Conn>, Json(book): Json<Book>) -> StatusCode {
    let query = "INSERT INTO book (title, isbn, author, publication_year, total_copies, available_copies) VALUES ($1, $2, $3, $4, $5, $6)";
    sqlx::query(query)
        .bind(book.title)
        .bind(book.isbn)
        .bind(book.author)
        .bind(book.publication_year)
        .bind(book.total_copies)
        .bind(book.available_copies)
        .execute(&*pool)
        .await
        .unwrap();

    StatusCode::CREATED
}

#[axum_macros::debug_handler]
async fn get_books(State(pool): State<Conn>) -> Json<Vec<Book>> {
    let query =
        "SELECT title, author, isbn, publication_year, total_copies, available_copies FROM book";
    let books = sqlx::query_as::<_, Book>(query)
        .fetch_all(&(*pool))
        .await
        .unwrap();

    Json(books)
}

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize, FromRow)]
struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub publication_year: i32,
    pub total_copies: i32,
    pub available_copies: i32,
}
