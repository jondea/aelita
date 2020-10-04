
CREATE TABLE cases(
  id uuid NOT NULL PRIMARY KEY,
  data jsonb NOT NULL
);

CREATE TABLE events(
  id uuid NOT NULL PRIMARY KEY,
  case_id uuid NOT NULL,
  data jsonb NOT NULL,
  CONSTRAINT fk_events_case_id
    FOREIGN KEY(case_id) 
      REFERENCES cases(id)
      ON DELETE CASCADE
);
