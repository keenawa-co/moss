CREATE TABLE "ignore_list" (
    "id" VARCHAR(20) PRIMARY KEY,
    "source" TEXT UNIQUE NOT NULL
);

CREATE INDEX "idx_source" ON ignore_list("source");

CREATE TABLE "manifest_serial"(
    "version" INTEGER PRIMARY KEY, 
    "hash" VARCHAR NOT NULL
);