# Home Lab Manager

Home Lab Manager is a self-hosted cybersecurity learning platform inspired by hands-on training products such as Hack The Box and TryHackMe. It combines structured Academy content with isolated VirtualBox machines, an SSH-backed browser terminal and Capture The Flag exercises.

> [!WARNING]
> This project is under active development and is not production-ready. Lab and Academy admin authorization still require hardening, and the committed migrations do not yet contain every column used by the current Lab repositories.

## Features

- User registration, login and JWT sessions.
- Role-aware backend middleware.
- Courses containing ordered sections and typed tasks.
- Academy workspace for lesson, practice and Lab tasks.
- Scenario-backed VirtualBox machine provisioning.
- Active Lab restoration after revisiting a task.
- Embedded xterm.js terminal over a WebSocket-to-SSH bridge.
- Flag validation, duplicate-score prevention and task completion tracking.
- Course, section and task administration APIs.

Some experiences remain partial: Practice interaction, video/download/hint widgets, the leaderboard, user profiles and the admin frontend. See the [feature status](docs/FEATURES.md) and [roadmap](docs/ROADMAP.md).

## Architecture

```text
React + xterm.js
    | REST/JSON and WebSocket
    v
Axum -> handlers -> services -> repositories -> PostgreSQL
                     |
                     +-> VirtualBox -> SSH -> Lab VM
```

The backend follows a routes/middleware/handlers/services/repositories structure. The browser uses REST for application data and Lab lifecycle operations, then opens a WebSocket terminal addressed by environment ID. The backend resolves the associated instance and bridges the connection to SSH.

For the full design, see [System Architecture](docs/ARCHITECTURE.md).

## Technology stack

### Frontend

- React 19
- JavaScript and JSX
- Vite
- React Router
- Tailwind CSS
- xterm.js

### Backend

- Rust 2024 edition
- Axum and Tokio
- SQLx and PostgreSQL
- JWT and bcrypt
- WebSockets and SSH

### Lab infrastructure

- Linux host
- VirtualBox and `VBoxManage`
- Cloneable VM templates with SSH access

## Prerequisites

Install the following before running the project:

- Node.js and npm compatible with Vite 8.
- A current stable Rust toolchain and Cargo.
- PostgreSQL.
- SQLx CLI if migrations will be run from the command line.
- VirtualBox with `VBoxManage` available on `PATH` for Lab functionality.
- A registered VM template matching the `vm_template_name` stored for each scenario.

The Academy and authentication portions can be developed without starting a VM, but Lab creation and the terminal require VirtualBox and a compatible SSH-enabled template.

## Configuration

### Backend

Create `backend/.env`:

```dotenv
DATABASE_URL=postgres://<user>:<password>@localhost:5432/<database>
JWT_SECRET=<long-random-secret>
```

Do not commit real credentials. The repository currently has no sanitized `.env.example`; adding one is recommended.

The current terminal handler contains development SSH credentials in source code. Replace this with environment variables or a secret store before using the project outside a local development environment.

### Frontend

The frontend defaults to `http://localhost:3000`. To use another backend origin, create `frontend/.env`:

```dotenv
VITE_API_URL=http://localhost:3000
```

The WebSocket origin is derived from this value by changing `http`/`https` to `ws`/`wss`.

## Database setup

Create a PostgreSQL database, configure `DATABASE_URL`, then run the migrations from the backend directory:

```bash
cd backend
sqlx migrate run
```

> [!IMPORTANT]
> The current migrations do not fully reproduce the schema expected by the Lab repositories. A forward migration is still required for scenario, environment and instance columns, as well as the `network_name` mismatch. Authentication and Academy setup may work, but do not expect a clean checkout to run the complete Lab lifecycle until this is resolved. Details are in [Database](docs/DATABASE.md).

## Running locally

Start the backend:

```bash
cd backend
cargo run
```

The API listens on `http://localhost:3000`.

In another terminal, install and start the frontend:

```bash
cd frontend
npm ci
npm run dev
```

Open the local URL printed by Vite.

## Development commands

Frontend:

```bash
cd frontend
npm run lint
npm run build
```

Backend:

```bash
cd backend
cargo fmt --check
cargo check
cargo test
```

SQLx queries may require a reachable database whose schema matches the repositories unless offline query metadata is configured.

## Project structure

```text
home-lab-manager/
├── backend/
│   ├── migrations/        PostgreSQL migrations
│   └── src/
│       ├── routes/        API route definitions
│       ├── middleware/    Authentication and admin checks
│       ├── handlers/      HTTP and WebSocket handlers
│       ├── services/      Business logic and orchestration
│       ├── repositories/  SQL queries
│       ├── models/        Entities, DTOs and statuses
│       └── utils/         VirtualBox, SSH and network helpers
├── frontend/
│   └── src/
│       ├── features/      Auth, Academy, Labs and CTF
│       ├── layouts/       Public and authenticated shells
│       ├── routes/        Frontend route guards
│       └── shared/        Reusable UI components
└── docs/                  Detailed project documentation
```

## Documentation

- [Project Overview](docs/PROJECT_OVERVIEW.md)
- [System Architecture](docs/ARCHITECTURE.md)
- [Backend](docs/BACKEND.md)
- [Frontend](docs/FRONTEND.md)
- [Database](docs/DATABASE.md)
- [API Reference](docs/API.md)
- [Feature Status](docs/FEATURES.md)
- [Roadmap](docs/ROADMAP.md)
- [Technical Decisions](docs/DECISIONS.md)

## Known limitations

- Academy admin middleware is implemented but not enabled on its routes.
- Lab HTTP and WebSocket routes are not protected by backend authentication middleware.
- Lab requests currently trust a client-provided `user_id` instead of deriving identity from JWT claims.
- Database migrations and current Lab repository queries are out of sync.
- CORS does not currently allow PUT even though Academy updates use it.
- VM provisioning is synchronous and may hold a request while waiting up to 120 seconds for SSH.
- Practice tasks and several content widgets are incomplete.
- The standalone Machines page uses hard-coded scenario UUIDs.
- Automated test coverage and production deployment guidance are still missing.

Track the planned fixes in the [Roadmap](docs/ROADMAP.md).
