CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE short_url (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    url TEXT NOT NULL,
    custom_id TEXT UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);