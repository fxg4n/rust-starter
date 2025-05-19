DROP INDEX IF EXISTS idx_users_username;
DROP INDEX IF EXISTS idx_users_email;
DROP INDEX IF EXISTS idx_sessions_user_id;
DROP INDEX IF EXISTS idx_auth_events_user_id;
DROP INDEX IF EXISTS idx_auth_events_type;
DROP INDEX IF EXISTS idx_verifications_user_id;
DROP INDEX IF EXISTS idx_password_audit_user_id;

DROP TABLE IF EXISTS password_audit;
DROP TABLE IF EXISTS verifications;
DROP TABLE IF EXISTS auth_events;
DROP TABLE IF EXISTS sessions;
DROP TABLE IF EXISTS users;