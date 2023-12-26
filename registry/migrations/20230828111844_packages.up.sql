CREATE TYPE package_type AS ENUM('library', 'api');

CREATE TABLE packages (
    id         SERIAL PRIMARY KEY,
    -- metadata
    name       TEXT NOT NULL UNIQUE,
    type       package_type NOT NULL, 
    -- timestamps
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE package_owners (
    id         SERIAL PRIMARY KEY,
    -- references
    user_id    INTEGER NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    package_id INTEGER NOT NULL REFERENCES packages(id) ON DELETE RESTRICT,
    invited_by INTEGER REFERENCES users(id) ON DELETE RESTRICT,
    -- timestamps
    created_at TIMESTAMPTZ NOT NULL,
    deleted_at TIMESTAMPTZ
);

CREATE TABLE package_invites (
    id SERIAL PRIMARY KEY,

    accepted   BOOLEAN,
    answered_at TIMESTAMPTZ,

    user_id    INTEGER NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    package_id INTEGER NOT NULL REFERENCES packages(id) ON DELETE RESTRICT,
    invited_by INTEGER NOT NULL REFERENCES users(id) ON DELETE RESTRICT,

    created_at TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL
);
