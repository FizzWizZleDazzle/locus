-- Replace inline question_image blob (compressed SVG) with a URL pointing
-- to an object in the diagrams bucket (MinIO). No legacy compatibility:
-- existing rows lose their inline diagram.
ALTER TABLE problems DROP COLUMN question_image;
ALTER TABLE problems ADD COLUMN question_image_url TEXT NOT NULL DEFAULT '';
