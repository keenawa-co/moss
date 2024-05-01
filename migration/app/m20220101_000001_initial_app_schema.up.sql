CREATE TABLE project_meta (
    id VARCHAR(20) PRIMARY KEY NOT NULL,
    source TEXT UNIQUE NOT NULL,
    created_at TIMESTAMP NOT NULL
);

CREATE INDEX idx_source ON project_meta(source);

CREATE TABLE session (
    id VARCHAR(20) PRIMARY KEY NOT NULL,
    project_meta_id VARCHAR(20) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    FOREIGN KEY(project_meta_id) REFERENCES project_meta(id)
);

CREATE INDEX idx_project_meta_id ON session(project_meta_id);