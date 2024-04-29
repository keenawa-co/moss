CREATE TABLE project (
    id VARCHAR(20) PRIMARY KEY NOT NULL,
    source TEXT UNIQUE NOT NULL,
    created_at TIMESTAMP NOT NULL
);

CREATE TABLE session (
    id VARCHAR(20) PRIMARY KEY NOT NULL,
    project_id VARCHAR(20) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    FOREIGN KEY(project_id) REFERENCES project(id)
);

CREATE INDEX idx_project_id ON session(project_id);