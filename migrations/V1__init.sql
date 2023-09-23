CREATE TABLE pessoas (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    apelido varchar(32) UNIQUE NOT NULL,
    nome varchar(100) NOT NULL,
    nascimento varchar(10) NOT NULL CHECK (
        nascimento ~ '^\d{4}\-(0?[1-9]|1[012])\-(0?[1-9]|[12][0-9]|3[01])$'
    ),
    stack varchar(32)[]
);