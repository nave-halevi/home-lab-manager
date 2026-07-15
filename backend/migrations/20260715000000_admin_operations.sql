ALTER TABLE users
ADD COLUMN IF NOT EXISTS is_active BOOLEAN NOT NULL DEFAULT TRUE;

CREATE TABLE IF NOT EXISTS admin_activity_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    admin_user_id UUID NOT NULL REFERENCES users(id),
    action VARCHAR(100) NOT NULL,
    entity_type VARCHAR(100) NOT NULL,
    entity_id UUID NULL,
    details JSONB NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_admin_activity_logs_created_at
ON admin_activity_logs (created_at DESC);

CREATE INDEX IF NOT EXISTS idx_admin_activity_logs_admin_user_id
ON admin_activity_logs (admin_user_id);

CREATE INDEX IF NOT EXISTS idx_admin_activity_logs_entity_type
ON admin_activity_logs (entity_type);

CREATE INDEX IF NOT EXISTS idx_users_is_active
ON users (is_active);
