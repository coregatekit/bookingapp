-- Your SQL goes here
CREATE TABLE
  IF NOT EXISTS zones (
    id UUID PRIMARY KEY DEFAULT uuidv7 (),
    label VARCHAR(50) NOT NULL,
    price NUMERIC(10, 2) NOT NULL,
    total_seats INT NOT NULL,
    event_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    CONSTRAINT fk_event FOREIGN KEY (event_id) REFERENCES events (id) ON DELETE CASCADE
  );