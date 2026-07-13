# System Architecture

## Runtime overview

```text
React application
    | REST/JSON + JWT
    v
Axum routes -> middleware -> handlers -> services -> repositories -> PostgreSQL
                                 |
                                 +-> VirtualBox lifecycle

Browser xterm.js
    | WebSocket
    v
Axum terminal handler -> SSH session -> Lab virtual machine
```

## Frontend

The React application is organized by feature:

- `features/auth`: registration, login and local session restoration.
- `features/academy`: course catalog, course workspace and typed task layouts.
- `features/labs`: Lab API client and machine state hooks.
- `features/ctf`: terminal and flag-submission behavior.
- `shared/ui`: reusable visual components.

`RequireAuth` protects application pages in the browser. This improves navigation behavior but is not a substitute for backend authorization.

The Academy workspace selects a task from the course hierarchy and delegates rendering to `TaskRenderer`. Lab state is scoped to `useLabs`; `LabLayout` restores the active environment for the selected scenario from the backend.

## Backend layers

### Routes

Routes define the public HTTP surface under `/api`. Authentication, user, Academy and Lab routes are separate modules.

### Middleware

The authentication middleware validates a Bearer JWT and adds its claims to the request. The admin middleware requires an authenticated claim with the `admin` role.

The user-list route uses both middleware layers. Academy admin and Lab routes currently do not apply them; this is a known security gap.

### Handlers

Handlers deserialize request DTOs, call a service and convert the result into an HTTP response. Some handlers currently map several distinct service failures to a single status code.

### Services

Services contain orchestration and business rules. For example, the instance service validates scenarios, coordinates database state and VirtualBox, validates flags and records task completion.

### Repositories

Repositories own SQL operations for users, Academy data, scenarios, environments, instances, flags and task progress.

## Lab creation sequence

```text
POST /api/lab/create
    -> validate active scenario
    -> reject an existing active environment
    -> insert environment with Building status
    -> allocate an SSH host port
    -> insert instance with Starting status
    -> clone the configured VirtualBox template
    -> start VM and configure port forwarding
    -> wait up to 120 seconds for SSH
    -> mark instance and environment Running
    -> return environment ID and SSH port
```

Failures mark the environment or instance as `Failed` and attempt to remove the cloned VM. Deletion transitions records through stopping states and removes the VirtualBox machine.

## Terminal sequence

The browser opens `/api/lab/terminal/:environment_id` as a WebSocket. The handler resolves the instance and SSH port from the environment ID, verifies that it is running, establishes an SSH connection and bridges terminal input/output.

## Data ownership and trust boundary

The current Lab API accepts `user_id` from request bodies and route parameters. Although the frontend sends a JWT, Lab routes do not validate it. The intended architecture is to derive the user identity from verified JWT claims and treat client-provided identifiers only as resource identifiers.
