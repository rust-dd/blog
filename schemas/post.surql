DEFINE TABLE IF NOT EXISTS post TYPE NORMAL SCHEMAFULL 
    PERMISSIONS 
        FOR select FULL
        FOR create, update, delete NONE;

DEFINE FIELD IF NOT EXISTS title ON post TYPE string ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS summary ON post TYPE string ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS body ON post TYPE string ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS slug ON post TYPE option<string>;
DEFINE FIELD IF NOT EXISTS tags ON post TYPE array<string> ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS author ON post TYPE record<author> ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS read_time ON post TYPE option<int>;
DEFINE FIELD IF NOT EXISTS total_views ON post TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS created_at ON post TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON post TYPE datetime VALUE time::now();
DEFINE FIELD IF NOT EXISTS publish_to_x ON post TYPE bool DEFAULT false;