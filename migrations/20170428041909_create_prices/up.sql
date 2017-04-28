CREATE TABLE prices (
       id INTEGER NOT NULL,
       ts TIMESTAMP NOT NULL,
       count INTEGER NOT NULL,
       lowest INTEGER NOT NULL,
       percentile_5 INTEGER NOT NULL,
       percentile_10 INTEGER NOT NULL,
       percentile_20 INTEGER NOT NULL,
       percentile_40 INTEGER NOT NULL,
       percentile_80 INTEGER NOT NULL,
       PRIMARY KEY(id, ts)
);
CREATE INDEX ON prices (ts);

