# Backend

Language:
- Rust

Framework:
- Axum

Async Runtime:
- Tokio

Database:
- PostgreSQL

Query Library:
- SQLx

Authentication:
- JWT

Routing:
- `/api/auth` for auth
- `/api/users` for admin user listing
- `/api/lab` for lab lifecycle
- `/api/academy` for course, section and task APIs

Architecture:

- Routes
- Handlers
- Services
- Repositories
- Database

The backend follows a layered architecture where every layer has a single responsibility.

Additional backend responsibilities:

- VirtualBox orchestration for lab VMs
- SSH port forwarding and terminal bridging
- Flag validation and user progress tracking
- Admin-protected academy CRUD APIs