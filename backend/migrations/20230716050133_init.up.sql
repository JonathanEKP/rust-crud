-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS products (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        product_name text NOT NULL,
        price text NOT NULL,
        quantity text NOT NULL
    );
