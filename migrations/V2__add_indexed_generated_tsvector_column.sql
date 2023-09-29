CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE OR REPLACE FUNCTION array_to_string_immut(text[])
RETURNS text LANGUAGE SQL IMMUTABLE AS $$
	SELECT array_to_string($1, ' ')
$$;

ALTER TABLE pessoas
ADD COLUMN row_text text GENERATED ALWAYS AS (
	lower(
		apelido || ' ' ||
		nome || ' ' ||
		replace(nascimento, '-', ' ') || ' ' ||
		array_to_string_immut(coalesce(stack, '{}'))
	)
) STORED;

CREATE INDEX pessoas_row_text_idx ON pessoas
USING GIST (row_text gist_trgm_ops(siglen=64));

/* ALTER TABLE pessoas
ADD COLUMN row_tsvector tsvector GENERATED ALWAYS AS (
	to_tsvector('simple',
		apelido || ' ' ||
		nome || ' ' ||
		replace(nascimento, '-', ' ') || ' ' ||
		array_to_string_immut(coalesce(stack, '{}'))
	)
) STORED;

CREATE INDEX pessoas_row_tsvector_idx ON pessoas
USING GIST (row_tsvector); */