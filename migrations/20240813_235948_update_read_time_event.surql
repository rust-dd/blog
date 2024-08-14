REMOVE FIELD read_time ON post;
DEFINE FIELD read_time ON post TYPE int DEFAULT 0;

REMOVE EVENT post_read_time ON TABLE post;
DEFINE EVENT post_read_time ON TABLE post WHEN $event = "UPDATE" && $before.body != $after.body THEN {
    LET $read_time = array::len(string::words($after.body)) / 200;
    UPDATE post SET read_time = math::max([1, $read_time]) WHERE id = $after.id;
};

