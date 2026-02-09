CREATE TABLE host
(
    id         INTEGER  NOT NULL PRIMARY KEY,
    name       TEXT     NOT NULL,
    port       INTEGER  NOT NULL,
    username   TEXT,
    password   TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_host_name_port ON host (name, port);

CREATE TABLE user
(
    id         INTEGER NOT NULL PRIMARY KEY,
    email      TEXT    NOT NULL,
    password   TEXT    NOT NULL,
    created_at TEXT    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_login TEXT,
    is_admin   BOOLEAN NOT NULL
);

CREATE UNIQUE INDEX idx_user_email ON user (email);

CREATE TABLE session
(
    id         INTEGER NOT NULL PRIMARY KEY,
    user_id    INTEGER NOT NULL,
    token      TEXT    NOT NULL,
    expires_at TEXT    NOT NULL,

    FOREIGN KEY (user_id) REFERENCES user (id) ON DELETE CASCADE
);

CREATE INDEX idx_session_expire_user_id ON session (expires_at, user_id);
CREATE INDEX idx_session_token ON session (token);

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

CREATE UNIQUE INDEX idx_permission_user_host_svc ON permission (user_id, host_id, service_name);
