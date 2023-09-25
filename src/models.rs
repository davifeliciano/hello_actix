use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use uuid::Uuid;

#[derive(Debug, Clone, PostgresMapper, Deserialize, Serialize, PartialEq)]
#[pg_mapper(table = "pessoas")]
pub struct Person {
    pub id: Option<Uuid>,
    pub apelido: String,
    pub nome: String,
    pub nascimento: String,
    pub stack: Option<Vec<String>>,
}
