DEFINE TABLE OVERWRITE reference SCHEMAFULL
    PERMISSIONS
        FOR select FULL
        FOR create, update, delete NONE;

DEFINE FIELD OVERWRITE title ON reference TYPE string;
DEFINE FIELD OVERWRITE description ON reference TYPE string;
DEFINE FIELD OVERWRITE url ON reference TYPE string;
DEFINE FIELD OVERWRITE tags ON reference TYPE array<string>;
DEFINE FIELD OVERWRITE tech_stack ON reference TYPE array<string>;
DEFINE FIELD OVERWRITE teck_stack_percentage ON reference TYPE array<int>;
DEFINE FIELD OVERWRITE is_published ON reference TYPE bool DEFAULT false;
DEFINE FIELD OVERWRITE created_at ON reference TYPE datetime DEFAULT time::now();
DEFINE FIELD OVERWRITE updated_at ON reference TYPE datetime VALUE time::now();
DEFINE FIELD OVERWRITE category ON reference TYPE string;
DEFINE FIELD OVERWRITE year ON reference TYPE string;
DEFINE FIELD OVERWRITE icon ON reference TYPE string;
