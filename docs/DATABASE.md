# Database

PostgreSQL stores authentication data, Academy content, Lab lifecycle state, flags, scores and task progress. UUIDs are used as primary keys.

## Relationships

```text
users
  ├── environments ── scenarios ── flags
  │        └── instances              └── user_flags ── users
  └── user_task_progress ── tasks

courses ── sections ── tasks ── optional scenario
```

## Tables

### `users`

- `id`: UUID primary key.
- `user_name`: display/login-related name.
- `email`: unique email.
- `password_hash`: bcrypt hash.
- `role`: `user` by default; the application also supports `admin`.
- `total_score`: cumulative CTF score.
- `created_at`, `updated_at`: timestamps.

### `courses`

- UUID, title, unique slug, optional description and difficulty.
- `is_published` controls visibility in the public catalog.
- Deleting a course cascades to sections and tasks.

### `sections`

- Belongs to a course.
- `order_index` is unique within a course.
- Contains title and optional description.

### `tasks`

- Belongs to a section.
- May reference a scenario.
- Contains title, content, task type, order and points.
- `order_index` is unique within a section.
- Current frontend types are `LESSON`, `PRACTICE` and `LAB`.

### `user_task_progress`

- Unique per user and task.
- Stores status, start time and completion time.
- Correct Lab flag submission upserts `COMPLETED`.

### `scenarios`

Represents a Lab definition and VM template. The current repositories expect:

- title, difficulty and description;
- `vm_template_name`;
- `estimated_time_minutes`;
- `max_score`;
- `is_active`.

Only active scenarios can create environments.

### `environments`

Represents one user's run of a scenario. The current repositories expect:

- user and scenario foreign keys;
- lifecycle status;
- creation, start, stop and last-activity timestamps.

Active lookup considers `Building`, `Running` and `Stopping` states.

### `instances`

Represents a VM within an environment. The current repositories expect:

- VM name and environment foreign key;
- entry-point flag and optional internal IP;
- host SSH port;
- lifecycle status;
- creation and last-activity timestamps.

### `flags`

- Belongs to a scenario.
- Stores an exact flag value and point value.

Production deployments should consider hashing or otherwise protecting flag values instead of storing them as plaintext.

### `user_flags`

- Joins a user and solved flag.
- Unique per user and flag to prevent duplicate scoring.
- Records the solve timestamp.

## Transactional scoring

Flag insertion and score increment run in one transaction. After this transaction succeeds, task progress is updated separately. Therefore a task-progress failure can occur after points have already been awarded; the service reports this case as an error and should eventually make the entire operation atomic.

## Migration mismatch

The committed migrations do not currently reproduce the schema expected by the Rust repositories.

Missing from the existing `scenarios` migration:

- `estimated_time_minutes`
- `max_score`
- `is_active`

Missing from the existing `environments` migration:

- `started_at`
- `stopped_at`
- `last_activity`

Missing from the existing `instances` migration:

- `ssh_port`
- `status`
- `last_activity`

The migration also defines `environments.network_name` as `NOT NULL`, while `create_environment` does not provide it. A new forward-only migration is required; existing migration files should not be rewritten after they have been applied to shared databases.

## Constraints to preserve

- Unique user email.
- Unique course slug.
- Unique section order within a course.
- Unique task order within a section.
- Unique progress row per user/task.
- Unique solved flag per user/flag.
- An appropriate uniqueness rule for active user/scenario environments should be enforced at the database level to prevent concurrent duplicate creation.
