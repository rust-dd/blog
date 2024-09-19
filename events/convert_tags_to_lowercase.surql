DEFINE EVENT convert_tags_to_lowercase ON TABLE post WHEN ($event = "CREATE" || $event = "UPDATE") && $before.tags != $after.tags THEN (
    UPDATE post SET tags = array::map($after.tags, |$v| string::lowercase($v)) WHERE id = $after.id
);