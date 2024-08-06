DEFINE TABLE author SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD id ON author TYPE record<author> ASSERT $value != NONE;
DEFINE FIELD name ON author TYPE string ASSERT $value != NONE;
DEFINE FIELD bio ON author TYPE option<string>;
DEFINE FIELD email ON author TYPE string ASSERT string::is::email($value);
DEFINE FIELD linkedin ON author TYPE option<string>;
DEFINE FIELD twitter ON author TYPE option<string>;
DEFINE FIELD github ON author TYPE option<string>;
