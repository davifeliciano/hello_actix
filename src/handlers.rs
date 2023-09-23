use crate::{database, errors::AppError, models::Person};
use actix_web::{get, http::header::ContentType, post, web, HttpResponse, Responder};
use deadpool_postgres::Pool;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct PersonSearchTerm {
    _t: Option<String>,
}

#[get("")]
pub async fn get_people(
    _query_params: web::Query<PersonSearchTerm>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    let client = pool.get().await.map_err(AppError::PoolError)?;
    let people = database::get_people(&client).await?;

    Ok(HttpResponse::Ok().json(people))
}

#[get("/{person_id}")]
pub async fn get_person(
    path: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    let id = Uuid::try_parse(&path.into_inner()).map_err(|_err| AppError::NotFound)?;
    let client = pool.get().await.map_err(AppError::PoolError)?;
    let person = database::get_person(&client, id).await?;

    Ok(HttpResponse::Ok().json(person))
}

#[get("/contagem-pessoas")]
pub async fn count_people(pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    let client = pool.get().await.map_err(AppError::PoolError)?;
    let count = database::count_people(&client).await?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(count.to_string()))
}

#[post("")]
pub async fn create_person(_body: web::Json<Person>, _pool: web::Data<Pool>) -> impl Responder {
    HttpResponse::Created()
}
