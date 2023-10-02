--
-- PostgreSQL database dump
--

-- Dumped from database version 14.9
-- Dumped by pg_dump version 14.9

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: pg_trgm; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS pg_trgm WITH SCHEMA public;


--
-- Name: EXTENSION pg_trgm; Type: COMMENT; Schema: -; Owner: -
--

COMMENT ON EXTENSION pg_trgm IS 'text similarity measurement and index searching based on trigrams';


--
-- Name: array_to_string_immut(text[]); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.array_to_string_immut(text[]) RETURNS text
    LANGUAGE sql IMMUTABLE
    AS $_$
	SELECT array_to_string($1, ' ')
$_$;


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: pessoas; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.pessoas (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    apelido character varying(32) NOT NULL,
    nome character varying(100) NOT NULL,
    nascimento character varying(10) NOT NULL,
    stack character varying(32)[],
    row_text text GENERATED ALWAYS AS (lower((((((((apelido)::text || ' '::text) || (nome)::text) || ' '::text) || replace((nascimento)::text, '-'::text, ' '::text)) || ' '::text) || public.array_to_string_immut((COALESCE(stack, '{}'::character varying[]))::text[])))) STORED,
    CONSTRAINT pessoas_nascimento_check CHECK (((nascimento)::text ~ '^\d{4}\-(0?[1-9]|1[012])\-(0?[1-9]|[12][0-9]|3[01])$'::text))
);


--
-- Name: refinery_schema_history; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.refinery_schema_history (
    version integer NOT NULL,
    name character varying(255),
    applied_on character varying(255),
    checksum character varying(255)
);


--
-- Name: pessoas pessoas_apelido_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.pessoas
    ADD CONSTRAINT pessoas_apelido_key UNIQUE (apelido);


--
-- Name: pessoas pessoas_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.pessoas
    ADD CONSTRAINT pessoas_pkey PRIMARY KEY (id);


--
-- Name: refinery_schema_history refinery_schema_history_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.refinery_schema_history
    ADD CONSTRAINT refinery_schema_history_pkey PRIMARY KEY (version);


--
-- Name: pessoas_row_text_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX pessoas_row_text_idx ON public.pessoas USING gist (row_text public.gist_trgm_ops (siglen='64'));


--
-- PostgreSQL database dump complete
--

