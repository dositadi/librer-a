use axum::{ Json, extract::{ Path, State }, http::StatusCode, response::IntoResponse };
use tracing::{ error, info };
use uuid::Uuid;

use crate::{ AppState, app::book::payload::{ BookRequest }, error::APIError, models::Book };

pub async fn list(State(mut state): State<AppState>) -> Result<impl IntoResponse, APIError> {
    let (limit, offset) = (10, 0);

    let books = Book::all()
        .limit(limit)
        .offset(offset)
        .exec(&mut state.db).await
        .map_err(|err| {
            error!(target: "database", "failed to fetch: {err:?}");
            APIError::ServerError
        })?;

    Ok((StatusCode::OK, Json(books)))
}

pub async fn create(
    State(mut state): State<AppState>,
    Json(payload): Json<BookRequest>
) -> Result<impl IntoResponse, APIError> {
    let saved = Book::create()
        .description(payload.description)
        .image_url(payload.image_url)
        .published_date(payload.published_date)
        .status(payload.status)
        .title(payload.title)
        .exec(&mut state.db).await
        .map_err(|err| {
            error!(target:"database", "failed to create book {err:?}");
            APIError::ServerError
        })?;

    info!(id = %saved.id, title = %saved.title, "new book created");

    Ok((StatusCode::CREATED, Json(saved)))
}

pub async fn read(
    State(mut state): State<AppState>,
    Path(id): Path<Uuid>
) -> Result<impl IntoResponse, APIError> {
    let book = Book::get_by_id(&mut state.db, id).await.map_err(|err| {
        error!(target: "database","failed to fetch book {err:?}");
        APIError::NotFound
    })?;

    Ok((StatusCode::OK, Json(book)))
}

pub async fn update(
    State(mut state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<BookRequest>
) -> Result<impl IntoResponse, APIError> {
    let book = Book::get_by_id(&mut state.db, id).await.map_err(|err| {
        if err.is_record_not_found() {
            APIError::NotFound
        } else {
            error!(target:"database","failed to fetch book");
            APIError::ServerError
        }
    })?;
    /* 
    toasty::update!(book {
        title: payload.title,
        description: payload.description,
        image_url: payload.image_url,
        published_date: payload.published_date,
        status: payload.status,
    })
    .exec(&mut state.db).await
        .map_err(|err| {
            error!(target: "database", "failed to update book {err:?}");
            APIError::ServerError
        })?; */

    Book::update_by_id(id)
        .description(payload.description)
        .status(payload.status)
        .created_at(book.created_at)
        .image_url(payload.image_url)
        .published_date(payload.published_date)
        .title(payload.title)
        .exec(&mut state.db).await
        .map_err(|err| {
            error!(target: "database", "failed to update book {err:?}");
            APIError::ServerError
        })?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete(
    State(mut state): State<AppState>,
    Path(id): Path<Uuid>
) -> Result<impl IntoResponse, APIError> {
    Book::delete_by_id(&mut state.db, id).await.map_err(|err| {
        if err.is_record_not_found() {
            APIError::NotFound
        } else if err.is_transaction_timeout() {
            APIError::RequestTimeout
        } else {
            error!(target: "database", "failed to delete product");
            APIError::ServerError
        }
    })?;

    Ok(StatusCode::NO_CONTENT)
}
