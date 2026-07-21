-- The app signs in as this database-level user instead of a cloud-managed
-- root user, so instance credential rotations can't take the service down.
DEFINE USER OVERWRITE blog_svc ON DATABASE
    PASSWORD "${svc_password}"
    ROLES EDITOR
    DURATION FOR TOKEN 1y, FOR SESSION NONE;
