DEFINE TABLE OVERWRITE post TYPE NORMAL SCHEMAFULL 
    PERMISSIONS 
        FOR select FULL
        FOR create, update, delete NONE;

DEFINE FIELD OVERWRITE title ON post TYPE string ASSERT $value != NONE;
DEFINE FIELD OVERWRITE summary ON post TYPE string ASSERT $value != NONE;
DEFINE FIELD OVERWRITE body ON post TYPE string ASSERT $value != NONE;
DEFINE FIELD OVERWRITE slug ON post TYPE option<string>;
DEFINE FIELD OVERWRITE tags ON post TYPE array<string> ASSERT $value != NONE;
DEFINE FIELD OVERWRITE author ON post TYPE record<author> ASSERT $value != NONE;
DEFINE FIELD OVERWRITE read_time ON post TYPE option<int>;
DEFINE FIELD OVERWRITE total_views ON post TYPE int DEFAULT 0;
DEFINE FIELD OVERWRITE created_at ON post TYPE datetime DEFAULT time::now();
DEFINE FIELD OVERWRITE updated_at ON post TYPE datetime VALUE time::now();
DEFINE FIELD OVERWRITE is_published ON post TYPE bool DEFAULT false;
DEFINE FIELD OVERWRITE header_image ON post TYPE option<string>;
DEFINE FIELD OVERWRITE show_cta ON post TYPE bool DEFAULT false;