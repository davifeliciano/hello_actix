use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;

use crate::{errors::AppError, models::Person};

pub async fn get_people(client: &Client) -> Result<Vec<Person>, AppError> {
    let people: Vec<Person> = client
        .query("SELECT * FROM pessoas LIMIT 50;", &[])
        .await?
        .iter()
        .map(|row| Person::from_row_ref(row).unwrap())
        .collect();

    Ok(people)
}

pub async fn get_person(client: &Client, id: Uuid) -> Result<Person, AppError> {
    let row = client
        .query_one("SELECT * FROM pessoas WHERE id = $1;", &[&id])
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
