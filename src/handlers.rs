use crate::{database, errors::AppError, models::Person, utils::validate_ymd_string};
use actix_web::{get, http::header::ContentType, post, web, HttpResponse};
use deadpool_postgres::Pool;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct PersonSearchTerm {
    t: String,
}

#[get("")]
pub async fn get_people(
    query_params: web::Query<PersonSearchTerm>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    let search_term = query_params.into_inner();
    let client = pool.get().await.map_err(AppError::PoolError)?;
    let people = database::get_people(&client, &search_term.t).await?;

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
pub async fn create_person(
    body: web::Json<Person>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    let person = body.into_inner();

    if !validate_ymd_string(&person.nascimento) {
        return Ok(HttpResponse::UnprocessableEntity()
            .body("Invalid date on field 'nascimento'. Expected format is YYYY-MM-DD."));
    };

    let client = pool.get().await.map_err(AppError::PoolError)?;

    client
        .query(
            r#"
                INSERT INTO pessoas
                    (apelido, nome, nascimento, stack)
                VALUES
                    ($1, $2, $3, $4);
            "#,
            &[
                &person.apelido,
                &person.nome,
                &person.nascimento,
                &person.stack,
            ],
        )
        .await
        .map_err(insert_error_mapper)?;

    Ok(HttpResponse::Created().into())
}

fn insert_error_mapper(err: tokio_postgres::Error) -> AppError {
    match err.as_db_error().map(|err| err.constraint()) {
        Some(Some(constraint)) => {
            if constraint == "pessoas_apelido_key" {
                AppError::Conflict
            } else {
                AppError::PostgresError(err)
            }
        }
        _ => AppError::PostgresError(err),
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{
        http::{header, StatusCode},
        test::{self},
        web::Bytes,
        App,
    };
    use deadpool_postgres::{Client, PoolError};
    use dotenv;
    use tokio_pg_mapper::FromTokioPostgresRow;
    use tokio_postgres::NoTls;

    use crate::config::Config;

    use super::*;

    fn get_pool() -> Pool {
        dotenv::from_filename(".env.test").unwrap();

        let config = Config::from_env().unwrap();
        config.pg.create_pool(None, NoTls).unwrap()
    }

    async fn clear_database(client: &Client) -> Result<(), PoolError> {
        client.batch_execute("TRUNCATE TABLE pessoas;").await?;

        Ok(())
    }

    async fn insert_person(client: &Client) -> Person {
        let row = client
            .query_one(
                r#"
                    INSERT INTO pessoas
                        (apelido, nome, nascimento, stack)
                    VALUES
                        ($1, $2, $3, $4)
                    RETURNING *;
                "#,
                &[
                    &"davifeliciano",
                    &"Davi Feliciano",
                    &"1999-02-18",
                    &vec!["Rust", "Python", "TypeScript"],
                ],
            )
            .await
            .unwrap();

        Person::from_row(row).unwrap()
    }

    fn app_config(pool: Pool) -> impl FnOnce(&mut web::ServiceConfig) {
        let json_config = Config::json_extractor_config();

        move |cfg: &mut web::ServiceConfig| {
            cfg.app_data(json_config)
                .app_data(web::Data::new(pool))
                .service(count_people)
                .service(
                    web::scope("/pessoas")
                        .service(get_people)
                        .service(get_person)
                        .service(create_person),
                );
        }
    }

    #[actix_web::test]
    async fn test_get_people() {
        let pool = get_pool();
        let client = pool.get().await.unwrap();

        clear_database(&client).await.unwrap();

        let app = App::new().configure(app_config(pool));
        let app = test::init_service(app).await;

        let inserted = insert_person(&client).await;

        let res = test::TestRequest::get()
            .uri("/pessoas")
            .send_request(&app)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let req = test::TestRequest::get().uri("/pessoas?t=rus").to_request();
        let result: Vec<Person> = test::call_and_read_body_json(&app, req).await;

        assert_eq!(vec![inserted], result);
    }

    #[actix_web::test]
    async fn test_get_person() {
        let pool = get_pool();
        let client = pool.get().await.unwrap();

        clear_database(&client).await.unwrap();

        let app = App::new().configure(app_config(pool));
        let app = test::init_service(app).await;

        let inserted = insert_person(&client).await;
        let uri = format!("/pessoas/{}", inserted.id.unwrap());
        let req = test::TestRequest::get().uri(&uri).to_request();
        let result: Person = test::call_and_read_body_json(&app, req).await;

        assert_eq!(inserted, result);
    }

    #[actix_web::test]
    async fn test_count_people() {
        let pool = get_pool();
        let client = pool.get().await.unwrap();

        clear_database(&client).await.unwrap();

        let app = App::new().configure(app_config(pool));
        let app = test::init_service(app).await;

        let req = test::TestRequest::get()
            .uri("/contagem-pessoas")
            .to_request();

        let result = test::call_and_read_body(&app, req).await;

        assert_eq!(result, Bytes::from_static(b"0"));

        insert_person(&client).await;

        let req = test::TestRequest::get()
            .uri("/contagem-pessoas")
            .to_request();

        let result = test::call_and_read_body(&app, req).await;

        assert_eq!(result, Bytes::from_static(b"1"));
    }

    fn create_person_request_factory(payload: String) -> test::TestRequest {
        test::TestRequest::post()
            .uri("/pessoas")
            .insert_header(header::ContentType::json())
            .set_payload(payload)
    }

    #[actix_web::test]
    async fn test_create_person() {
        let pool = get_pool();
        let client = pool.get().await.unwrap();

        clear_database(&client).await.unwrap();
        let inserted = insert_person(&client).await;

        let app = App::new().configure(app_config(pool));
        let app = test::init_service(app).await;

        let payload = format!(
            r#"
                {{
                    "apelido": "{}",
                    "nome": "Davi Feliciano",
                    "nascimento": "1999-02-18"
                }}
            "#,
            inserted.apelido
        );

        let req = create_person_request_factory(payload).to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::CONFLICT);

        let payload = r#"
            {
                "nome": "Davi Feliciano",
                "nascimento": "1999-02-18"
            }
        "#
        .to_string();

        let req = create_person_request_factory(payload).to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let payload = r#"
            {
                "apelido": "johndoe",
                "nome": "John Doe",
                "nascimento": "1970-01-01"
            }
        "#
        .to_string();

        let req = create_person_request_factory(payload).to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::CREATED);
    }
}
