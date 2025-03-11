-- Your SQL goes here
CREATE TABLE users (
                id pg_catalog.uuid PRIMARY KEY DEFAULT gen_random_uuid(),
                name VARCHAR(100) NOT NULL,
                email VARCHAR(255) NOT NULL UNIQUE,
                password VARCHAR(255) NOT NULL,
                role VARCHAR(10) NOT NULL DEFAULT 'user',
                phone VARCHAR(20),
                activated BOOLEAN DEFAULT FALSE NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);