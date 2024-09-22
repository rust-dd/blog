DEFINE EVENT IF NOT EXISTS post_slug ON TABLE post WHEN ($event = "CREATE" OR $event = "UPDATE") && $before.title != $after.title THEN (
    UPDATE post SET slug = string::slug($after.title) WHERE id = $after.id
);
