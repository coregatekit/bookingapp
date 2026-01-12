-- Your SQL goes here
CREATE TABLE
  IF NOT EXISTS seats (
    id UUID PRIMARY KEY DEFAULT uuidv7 (),
    seat_row VARCHAR(10) NOT NULL,
    seat_column VARCHAR(10) NOT NULL,
    seat_number VARCHAR(10) NOT NULL,
    is_reserved BOOLEAN NOT NULL DEFAULT FALSE,
    zone_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    CONSTRAINT fk_zone FOREIGN KEY (zone_id) REFERENCES zones (id) ON DELETE CASCADE
  )