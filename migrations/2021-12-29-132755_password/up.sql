-- Your SQL goes here
CREATE  TABLE  if not exists password (
    id INTEGER PRIMARY KEY,
    key VARCHAR  NOT NULL,
    value VARCHAR NOT NULL,
    description VARCHAR
);
CREATE INDEX idx ON password(key);