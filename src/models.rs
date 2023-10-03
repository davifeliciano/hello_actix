use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use uuid::Uuid;

use crate::utils::validate_ymd_string;

#[derive(Debug, Clone, PostgresMapper, Deserialize, Serialize, PartialEq)]
#[pg_mapper(table = "pessoas")]
pub struct Person {
    pub id: Option<Uuid>,
    pub apelido: String,
    pub nome: String,
    pub nascimento: String,
    pub stack: Option<Vec<String>>,
}

impl Person {
    pub const APELIDO_MAX_LEN: usize = 32;
    pub const NOME_MAX_LEN: usize = 100;
    pub const STACK_WORD_MAX_LEN: usize = 32;

    pub fn validate_apelido(&self) -> bool {
        self.apelido.len() <= Self::APELIDO_MAX_LEN
    }

    pub fn validate_nome(&self) -> bool {
        self.nome.len() <= Self::NOME_MAX_LEN
    }

    pub fn validate_nascimento(&self) -> bool {
        validate_ymd_string(&self.nascimento)
    }

    pub fn validate_stack(&self) -> bool {
        if let Some(stack) = &self.stack {
            for word in stack {
                if word.len() > Self::STACK_WORD_MAX_LEN {
                    return false;
                }
            }
        }

        true
    }

    pub fn get_error_message_if_not_valid(&self) -> Option<String> {
        let string_fields_messages = [
            format!("'apelido' must have up to {} chars", Self::APELIDO_MAX_LEN),
            format!("'nome' must have up to {} chars", Self::NOME_MAX_LEN),
            format!("'nascimento' must be a date with format YYYY-MM-DD"),
        ];

        let validators = [
            Self::validate_apelido,
            Self::validate_nome,
            Self::validate_nascimento,
        ];

        for (msg, validator) in string_fields_messages.iter().zip(validators.iter()) {
            if !validator(self) {
                return Some(msg.into());
            }
        }

        if !self.validate_stack() {
            return Some(format!(
                "'stack' must be a list with words having up to {} chars",
                Self::STACK_WORD_MAX_LEN
            ));
        }

        None
    }
}
