# Roadmap

## Completed foundations

- Rust/Axum backend and PostgreSQL repository layers.
- Registration, login, JWT generation and role claims.
- Academy course, section and task backend CRUD.
- Published course catalog and full-course frontend workspace.
- Typed task renderer for lesson, practice and Lab layouts.
- Scenario-backed VirtualBox machine creation and deletion.
- Environment and instance lifecycle models.
- Browser terminal through WebSocket and SSH.
- Active Lab restoration by user and scenario.
- Flag validation, duplicate prevention, scoring and task completion.

## Immediate: correctness and security

- Add a forward migration for the Lab columns currently required by repositories.
- Resolve the `network_name` mismatch between the migration and environment creation.
- Enable JWT and admin middleware for `/api/academy/admin`.
- Protect all Lab HTTP and WebSocket routes.
- Derive the acting user from JWT claims instead of accepting trusted `user_id` values.
- Add PUT to the allowed CORS methods.
- Map service failures to meaningful HTTP status codes and a shared error format.
- Make flag scoring and task-progress completion one atomic transaction.

## Near term: Lab reliability and UX

- Move VM provisioning to background jobs.
- Expose building, running, stopping and failure progress to the UI.
- Add provisioning cancellation and retry behavior.
- Add idle timeout, heartbeat and orphaned-VM cleanup.
- Reconcile database state with VirtualBox state after backend restart.
- Prevent port-allocation races.
- Define whether active Lab limits apply per scenario or globally.
- Add clear terminal reconnection behavior.

## Near term: Academy experience

- Complete the Practice interaction model.
- Implement video, download and hint widgets.
- Persist and display task progress throughout the course UI.
- Add locked/available/in-progress task behavior.
- Remove development logs and retire unused legacy Lab components.
- Replace the hard-coded Machines page with scenario data or remove it.

## Administration

- Build an admin dashboard.
- Add frontend management for courses, sections and tasks.
- Add scenario and VirtualBox-template management.
- Add validation and ordering controls.
- Add operational views for active, failed and orphaned environments.

## User and community features

- User profile and learning history.
- Real leaderboard based on stored scores.
- Statistics and course completion analytics.
- Achievements and certificates.

## Quality and operations

- Backend unit and integration tests.
- Frontend component and hook tests.
- End-to-end Academy Lab and flag tests.
- Structured logging and request tracing.
- Metrics for VM startup time, failures and active environments.
- Deployment, backup and disaster-recovery documentation.
- Secret management and production configuration guidance.
