# API Reference

The backend listens on port `3000` by default. All application endpoints are mounted below `/api` and use JSON unless the endpoint is identified as a WebSocket.

## Authentication header

Protected endpoints expect:

```http
Authorization: Bearer <jwt>
```

Current enforcement is limited: `/api/users` is protected, while Academy admin and Lab routes do not currently apply the available middleware. Clients should still send the token, but this must not be treated as server-side authorization until middleware is enabled.

## Authentication

### Register

```http
POST /api/auth/register
Content-Type: application/json

{
  "user_name": "student",
  "email": "student@example.com",
  "password": "secret"
}
```

Creates a user with a hashed password.

### Login

```http
POST /api/auth/login
Content-Type: application/json

{
  "email": "student@example.com",
  "password": "secret"
}
```

Returns a JWT and the authenticated user on success. The frontend stores both in `localStorage`.

## Users

### List users

```http
GET /api/users/
Authorization: Bearer <admin-jwt>
```

Requires both authentication and the `admin` role.

## Academy: public reads

### List published courses

```http
GET /api/academy/courses/
```

Returns the Academy course catalog.

### Get a full course

```http
GET /api/academy/courses/:id/full
```

Returns a course with its ordered sections and tasks.

## Academy: administration

The following routes are intended for administrators. The authorization layers in `academy_routes.rs` are currently commented out, so this is a known security issue.

### Courses

```http
POST   /api/academy/admin/courses
PUT    /api/academy/admin/courses/:id
DELETE /api/academy/admin/courses/:id
```

### Sections

```http
POST   /api/academy/admin/sections
GET    /api/academy/admin/sections/:id
PUT    /api/academy/admin/sections/:id
DELETE /api/academy/admin/sections/:id
GET    /api/academy/admin/courses/:course_id/sections
```

### Tasks

```http
POST   /api/academy/admin/tasks
GET    /api/academy/admin/tasks/:id
PUT    /api/academy/admin/tasks/:id
DELETE /api/academy/admin/tasks/:id
GET    /api/academy/admin/sections/:section_id/tasks
```

Course, section and task write bodies correspond to their DTO fields, including relationships, ordering, content, type, points and publication metadata. Update DTOs use optional fields for partial value selection, although the HTTP method is PUT.

## Lab engine

Lab endpoints currently accept a client-provided `user_id` and are not protected by backend middleware. This contract should be replaced by identity derived from JWT claims.

### Create a Lab

```http
POST /api/lab/create
Content-Type: application/json

{
  "user_id": "<uuid>",
  "scenario_id": "<uuid>"
}
```

Success: `201 Created`

```json
{
  "message": "The lab was set up and is running successfully!",
  "ssh_port": 2201,
  "env_id": "<environment-uuid>"
}
```

The request waits for cloning, startup and SSH readiness. It can take up to 120 seconds. The current handler returns `500 Internal Server Error` for all service failures, including an existing active environment or inactive scenario.

### Delete a Lab

```http
POST /api/lab/delete
Content-Type: application/json

{
  "user_id": "<uuid>",
  "env_id": "<environment-uuid>"
}
```

Returns `200 OK` after the VM is deleted and state is updated. Current failures return `500 Internal Server Error`.

### Get an active Lab for a scenario

```http
GET /api/lab/active/:user_id/:scenario_id
```

Returns `200 OK` with either `null` or a status object:

```json
{
  "environment_id": "<uuid>",
  "scenario_id": "<uuid>",
  "environment_status": "Running",
  "instance_id": "<uuid>",
  "vm_name": "lab-...",
  "ssh_port": 2201,
  "instance_status": "Running",
  "is_entry_point": true,
  "created_at": "<timestamp>",
  "started_at": "<timestamp-or-null>",
  "stopped_at": null
}
```

The frontend uses this endpoint to restore a running machine after revisiting a Lab task.

### Get Lab status

```http
GET /api/lab/status/:user_id/:environment_id
```

Returns `200 OK` with a status object when the user owns the environment. The current handler returns `404 Not Found` with `null` for lookup and service errors.

### Submit a flag

```http
POST /api/lab/submit
Content-Type: application/json

{
  "user_id": "<uuid>",
  "env_id": "<environment-uuid>",
  "task_id": "<task-uuid>",
  "flag": "CTF{example}"
}
```

The server verifies ownership, running status, the task/scenario relationship and the flag. Success and an incorrect flag both currently return `200 OK` with a human-readable message. Validation failures return `400 Bad Request`.

Examples:

```json
{ "message": "✅ Correct! You earned 10 points." }
```

```json
{ "message": "❌ Incorrect flag. Keep trying!" }
```

```json
{ "message": "⚠️ You already submitted this flag!" }
```

### Terminal WebSocket

```text
ws://localhost:3000/api/lab/terminal/:environment_id
```

This is a WebSocket upgrade endpoint, not a terminal endpoint addressed by SSH port. It resolves the entry-point instance for the environment and bridges WebSocket input/output to SSH. The instance must be running and have a valid SSH port.

## Status-code limitations

The API does not yet use a shared error envelope consistently. Several handlers return different JSON shapes or collapse unrelated errors into one status. Recommended future mappings include:

- `401` for a missing or invalid JWT.
- `403` for insufficient role or resource ownership.
- `404` for a missing course, scenario or environment.
- `409` for an already-active environment or duplicate ordering.
- `422` for semantically invalid request data.
- `500` for unexpected infrastructure or database failures.
