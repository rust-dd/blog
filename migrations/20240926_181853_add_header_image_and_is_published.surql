UPDATE post SET is_published = true;
REMOVE FIELD publish_to_x ON TABLE post;
UPDATE post;