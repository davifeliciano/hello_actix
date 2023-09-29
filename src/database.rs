use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;

use crate::{errors::AppError, models::Person};

pub async fn get_people(client: &Client, search_term: &str) -> Result<Vec<Person>, AppError> {
    let people: Vec<Person> = client
        .query(
            "
            SELECT id, apelido, nome, nascimento, stack
            FROM pessoas
            WHERE row_text %> $1
            LIMIT 50;
            ",
            &[&search_term],
        )
        .await?
        .iter()
        .map(|row| Person::from_row_ref(row).unwrap())
        .collect();

    Ok(people)
}

pub async fn get_person(client: &Client, id: Uuid) -> Result<Person, AppError> {
    let row = client
        .query_one(
            "
            SELECT id, apelido, nome, nascimento, stack
            FROM pessoas WHERE id = $1;
            ",
            &[&id],
        )
        .await?;

    if row.is_empty() {
        return Err(AppError::NotFound);
    }

    Ok(Person::from_row(row)?)
}

pub async fn count_people(client: &Client) -> Result<i64, AppError> {
    let count: i64 = client
        .query_one("SELECT count(*) AS count FROM pessoas;", &[])
        .await?
        .get("count");

    Ok(count)
}
