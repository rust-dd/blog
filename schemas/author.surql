DEFINE TABLE IF NOT EXISTS author TYPE NORMAL SCHEMAFULL
    PERMISSIONS 
        FOR select FULL
        FOR create, update, delete NONE;

DEFINE FIELD IF NOT EXISTS name ON author TYPE string ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS bio ON author TYPE option<string>;
DEFINE FIELD IF NOT EXISTS email ON author TYPE string ASSERT string::is::email($value);
DEFINE FIELD IF NOT EXISTS linkedin ON author TYPE option<string>;
DEFINE FIELD IF NOT EXISTS twitter ON author TYPE option<string>;
DEFINE FIELD IF NOT EXISTS github ON author TYPE option<string>;
