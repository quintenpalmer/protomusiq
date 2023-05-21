ALTER TABLE artists ADD COLUMN deleted_at TEXT;
ALTER TABLE albums  ADD COLUMN deleted_at TEXT;
ALTER TABLE discs   ADD COLUMN deleted_at TEXT;
ALTER TABLE tracks  ADD COLUMN deleted_at TEXT;
