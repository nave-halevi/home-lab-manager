# Backend

## Stack

- Rust 2024 edition.
- Axum 0.7 for REST and WebSocket routes.
- Tokio for the asynchronous runtime.
- PostgreSQL and SQLx.
- JWT (`jsonwebtoken`) and bcrypt authentication.
- `ssh2` for terminal sessions.
- VirtualBox command-line integration for VM lifecycle.

The server listens on `0.0.0.0:3000` and mounts application routes below `/api`.

## Source structure

```text
src/
├── routes/        HTTP route definitions
├── middleware/    JWT and role authorization
├── handlers/      HTTP request/response mapping
├── services/      Business logic and orchestration
├── repositories/  SQL queries
├── models/        Entities, DTOs, request models and statuses
├── utils/         VirtualBox, SSH, port and system helpers
└── errors/        Shared application error type
```

## Route groups

- `/api/auth`: register and login.
- `/api/users`: admin-protected user listing.
- `/api/academy/courses`: public course reads.
- `/api/academy/admin`: course, section and task management.
- `/api/lab`: creation, deletion, status, active-environment lookup, flag submission and terminal WebSocket.

The exact contracts are documented in [API](API.md).

## Authentication and authorization

The authentication middleware expects `Authorization: Bearer <token>`, verifies it and stores claims on the request. The admin middleware reads those claims and requires the `admin` role.

Current protection:

- `/api/users`: authentication and admin role required.
- `/api/academy/admin`: middleware exists but is currently commented out.
- `/api/lab`: currently has no backend authentication middleware.
- Public Academy reads and auth endpoints: public.

Consequently, the Academy admin and Lab routes must not yet be considered production-secure. Lab operations currently trust a client-supplied `user_id`.

## Academy service

The Academy service provides:

- Published course listing and full course aggregation.
- Course create, update and delete.
- Ordered section create, read, update and delete.
- Ordered task create, read, update and delete.
- Conversion between database entities and API DTOs.

A full course response groups joined rows into their course, sections and tasks hierarchy.

## Lab service

### Creation

Creation validates an active scenario, checks for an existing active environment, creates database records, allocates a host SSH port, clones and starts the VM, waits for SSH readiness and marks the records as running.

The HTTP request remains open while provisioning and may wait up to 120 seconds for SSH. A future job-based flow is listed in the roadmap.

### Restoration and status

The service can return an active environment by user and scenario or return status for a known environment. These responses include environment and instance state, VM name, SSH port and timestamps where available.

### Deletion

Deletion verifies that the environment belongs to the supplied user, marks it as stopping, deletes the VirtualBox VM and records the terminal states in PostgreSQL.

### Flag submission

Submission requires a running user environment and a `LAB` task attached to the same scenario. A correct flag is inserted transactionally into `user_flags` and increments `users.total_score`. The task is then marked `COMPLETED`. A unique constraint prevents duplicate scoring.

## Status values

Environment states used by the application:

- `Building`
- `Running`
- `Stopping`
- `Destroyed`
- `Failed`

Instance states used by the application:

- `Starting`
- `Running`
- `Stopping`
- `Stopped`
- `Failed`

## VirtualBox and SSH assumptions

- The backend host has VirtualBox and `VBoxManage` installed.
- Every active scenario references a registered VM template.
- The template accepts the SSH credentials expected by the SSH helper.
- A host port can be allocated and forwarded to guest SSH.
- The backend process has permission to clone, start and delete VMs.

## Configuration

The backend requires `DATABASE_URL`. JWT and SSH-related configuration should be reviewed in the associated service and utility modules before deployment; secrets must not be committed.

## Known backend issues

- Academy admin and Lab routes require backend authorization middleware.
- Lab user identity should come from JWT claims rather than request JSON.
- CORS currently allows GET, POST and DELETE but not PUT, although Academy update endpoints use PUT.
- Service errors are not consistently mapped to semantic HTTP status codes.
- Provisioning is synchronous and can hold a request for up to 120 seconds.
- The current migrations do not fully match repository expectations; see [Database](DATABASE.md).
