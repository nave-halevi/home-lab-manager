# Features

This document distinguishes complete, partial and planned behavior. A source file existing in the repository does not necessarily mean the feature is complete or production-ready.

## Implemented

### Authentication

- User registration with bcrypt password hashing.
- Email/password login.
- JWT generation and verification.
- Frontend session restoration from local storage.
- Protected frontend application routes.
- Role-aware middleware for backend routes.
- Admin-protected user listing.

### Academy backend

- Published course catalog.
- Full course response containing ordered sections and tasks.
- Course create, update and delete.
- Section create, read, update and delete.
- Task create, read, update and delete.
- Optional task-to-scenario relationship.

### Academy frontend

- Course catalog and course cards.
- Full course workspace.
- Section/task sidebar and initial task selection.
- Generic task renderer for `LESSON`, `PRACTICE` and `LAB`.
- Learning and interaction panels.
- Markdown/text lesson content.

### Lab lifecycle

- Scenario validation.
- Per-user/per-scenario active environment lookup.
- Environment and instance state tracking.
- Available host-port allocation.
- VirtualBox template cloning.
- VM startup and SSH-readiness wait.
- Failure cleanup.
- Active Lab restoration in the Academy workspace.
- VM termination and record-state updates.

### Terminal

- Embedded xterm.js terminal.
- WebSocket connection by environment ID.
- Backend WebSocket-to-SSH bridge.
- Bidirectional terminal input/output.
- Terminal resize and component cleanup.

### CTF, score and progress

- Flag input in Lab tasks.
- Environment ownership and running-state validation.
- Task-to-scenario validation.
- Exact flag verification.
- Transactional flag recording and score increment.
- Duplicate-score prevention.
- Task completion record after a correct or previously solved flag.

## Partial or prototype features

### Authorization

JWT and admin middleware are implemented, but Academy admin and Lab routes do not currently apply them. Those routes are not production-secure.

### Practice tasks

The Practice layout exists, but its terminal is not connected to an active Lab environment.

### Additional content widgets

`VideoWidget`, `DownloadWidget` and `HintWidget` exist as empty placeholders.

### Machines page

The `/machines` route displays selectable machines but uses hard-coded scenario UUIDs. It is separate from the primary Academy Lab experience.

### Dashboard and leaderboard

The dashboard contains static/demo content. The leaderboard route is a placeholder, even though users already have a stored total score.

### Lab status UX

Active restoration exists, but provisioning is a single long-running request with only a general loading state. There is no queued/building progress view, timeout recovery interface or automatic idle cleanup.

### Database setup

Migrations exist, but they do not yet include all columns required by the current Lab repositories.

## Planned or missing

- User profile page.
- Admin frontend for users, courses, sections, tasks and scenarios.
- Scenario and VM-template management UI/API.
- Real leaderboard and statistics.
- Certificates and achievements.
- Complete video, download, hint and practice experiences.
- Application-wide Lab state and lifecycle notifications.
- Asynchronous provisioning jobs and progress polling.
- Idle timeout and orphaned-VM cleanup.
- Production authorization for every write and user-owned resource.
- Automated backend, frontend and end-to-end test coverage.
