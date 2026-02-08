CREATE TABLE host
(
    id         INTEGER PRIMARY KEY,
    name       TEXT,
    port       INTEGER,
    username   TEXT,
    password   TEXT,
    created_at TEXT,
    updated_at TEXT
);

CREATE INDEX idx_host_name ON host (name);

CREATE TABLE user
(
    id         INTEGER PRIMARY KEY,
    email      TEXT,
    password   TEXT,
    created_at TEXT,
    updated_at TEXT,
    last_login TEXT,
    is_admin   BOOLEAN
);

CREATE INDEX idx_user_email ON user (email);

CREATE TABLE session
(
    id         INTEGER PRIMARY KEY,
    user_id    INTEGER,
    token      TEXT,
    expires_at TEXT,

    FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
);

CREATE INDEX idx_session_expire_user_id ON session (expires_at, user_id);

CREATE TABLE permission
(
    id           INTEGER PRIMARY KEY,
    user_id      INTEGER,
    host_id      INTEGER,
    service_name TEXT,
    can_view     BOOLEAN,
    can_act      BOOLEAN,

    FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE,
    FOREIGN KEY (host_id) REFERENCES host(id) ON DELETE CASCADE
);

CREATE INDEX idx_permission_user_host ON permission (user_id, host_id);
