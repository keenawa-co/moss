CREATE TABLE project (
    id VARCHAR(20) PRIMARY KEY NOT NULL,
    source TEXT UNIQUE NOT NULL,
    last_used_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL
);

CREATE TABLE session (
    id VARCHAR(20) PRIMARY KEY NOT NULL,
    project_id VARCHAR(20) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    FOREIGN KEY(project_id) REFERENCES project(id)
);

CREATE TABLE metric_feed (
    id VARCHAR(20) PRIMARY KEY NOT NULL,
    session_id VARCHAR(20) NOT NULL,
    project_id VARCHAR(20) NOT NULL,
    feed JSON,
    chunk_num INT NOT NULL,
    chunk_total INT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    FOREIGN KEY(session_id) REFERENCES session(id),
    FOREIGN KEY(project_id) REFERENCES project(id)
);

CREATE INDEX idx_session_id ON metric_feed(session_id);
CREATE INDEX idx_project_id ON metric_feed(project_id);