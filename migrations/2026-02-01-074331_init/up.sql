CREATE TABLE host
(
    id         INTEGER  NOT NULL PRIMARY KEY,
    name       TEXT     NOT NULL,
    port       INTEGER  NOT NULL,
    username   TEXT,
    password   TEXT,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

CREATE INDEX idx_host_name ON host (name);

CREATE TABLE user
(
    id         INTEGER NOT NULL PRIMARY KEY,
    email      TEXT    NOT NULL,
    password   TEXT    NOT NULL,
    created_at TEXT    NOT NULL,
    updated_at TEXT    NOT NULL,
    last_login TEXT,
    is_admin   BOOLEAN NOT NULL
);

CREATE INDEX idx_user_email ON user (email);

CREATE TABLE session
(
    id         INTEGER NOT NULL PRIMARY KEY,
    user_id    INTEGER NOT NULL,
    token      TEXT    NOT NULL,
    expires_at TEXT    NOT NULL,

    FOREIGN KEY (user_id) REFERENCES user (id) ON DELETE CASCADE
);

CREATE INDEX idx_session_expire_user_id ON session (expires_at, user_id);

CREATE TABLE permission
(
    id           INTEGER NOT NULL PRIMARY KEY,
    user_id      INTEGER NOT NULL,
    host_id      INTEGER NOT NULL,
    service_name TEXT    NOT NULL,
    can_view     BOOLEAN NOT NULL,
    can_act      BOOLEAN NOT NULL,

    FOREIGN KEY (user_id) REFERENCES user (id) ON DELETE CASCADE,
    FOREIGN KEY (host_id) REFERENCES host (id) ON DELETE CASCADE
);

CREATE INDEX idx_permission_user_host ON permission (user_id, host_id);
