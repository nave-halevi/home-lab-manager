ALTER TABLE user_task_progress
ADD COLUMN IF NOT EXISTS earned_points INT NOT NULL DEFAULT 0;

UPDATE user_task_progress AS progress
SET earned_points = tasks.points
FROM tasks
WHERE progress.task_id = tasks.id
  AND progress.status = 'COMPLETED'
  AND progress.earned_points = 0;

UPDATE users
SET total_score = COALESCE((
    SELECT SUM(progress.earned_points)
    FROM user_task_progress AS progress
    WHERE progress.user_id = users.id
), 0);

ALTER TABLE environments
ADD COLUMN IF NOT EXISTS expires_at TIMESTAMPTZ;

UPDATE environments
SET expires_at = COALESCE(expires_at, last_activity + INTERVAL '20 minutes')
WHERE status IN ('Building', 'Running', 'Stopping');

CREATE UNIQUE INDEX IF NOT EXISTS idx_environments_one_active_lab_per_user
ON environments (user_id)
WHERE status IN ('Building', 'Running', 'Stopping');

CREATE INDEX IF NOT EXISTS idx_environments_expires_at_active
ON environments (expires_at)
WHERE status IN ('Building', 'Running', 'Stopping');
