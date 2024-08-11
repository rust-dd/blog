DEFINE TABLE post TYPE NORMAL SCHEMAFULL 
    PERMISSIONS 
        FOR select FULL
        FOR create, update, delete NONE;

DEFINE FIELD id ON post TYPE record<post> ASSERT $value != NONE;
DEFINE FIELD title ON post TYPE string ASSERT $value != NONE;
DEFINE FIELD summary ON post TYPE string ASSERT $value != NONE;
DEFINE FIELD body ON post TYPE string ASSERT $value != NONE;
DEFINE FIELD slug ON post TYPE option<string>;
DEFINE FIELD tags ON post TYPE array<string> ASSERT $value != NONE;
DEFINE FIELD author ON post TYPE record<author> ASSERT $value != NONE;
DEFINE FIELD read_time ON post TYPE int ASSERT $value != NONE;
DEFINE FIELD total_views ON post TYPE int DEFAULT 0;
DEFINE FIELD created_at ON post TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON post TYPE datetime VALUE time::now();