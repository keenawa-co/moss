CREATE TABLE project (
    id VARCHAR(20) PRIMARY KEY NOT NULL,
    source TEXT UNIQUE NOT NULL,
    last_used_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL
);

CREATE TABLE project_watch_list (
    id VARCHAR(20) PRIMARY KEY NOT NULL,
    project_id VARCHAR(20) NOT NULL,
    source TEXT UNIQUE NOT NULL,
    FOREIGN KEY(project_id) REFERENCES project(id)
);

CREATE INDEX idx_project_id ON project_watch_list(project_id);