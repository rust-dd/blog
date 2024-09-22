DEFINE TABLE OVERWRITE author TYPE NORMAL SCHEMAFULL
    PERMISSIONS 
        FOR select FULL
        FOR create, update, delete NONE;

DEFINE FIELD OVERWRITE name ON author TYPE string ASSERT $value != NONE;
DEFINE FIELD OVERWRITE bio ON author TYPE option<string>;
DEFINE FIELD OVERWRITE email ON author TYPE string ASSERT string::is::email($value);
DEFINE FIELD OVERWRITE linkedin ON author TYPE option<string>;
DEFINE FIELD OVERWRITE twitter ON author TYPE option<string>;
DEFINE FIELD OVERWRITE github ON author TYPE option<string>;
