DEFINE EVENT IF NOT EXISTS post_read_time ON TABLE post WHEN ($event = "UPDATE" || $event = "CREATE") && $before.body != $after.body THEN {
    LET $read_time = array::len(string::words($after.body)) / 200;
    UPDATE post SET read_time = math::max([1, $read_time]) WHERE id = $after.id;
};
