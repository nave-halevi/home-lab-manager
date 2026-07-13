# Technical Decisions

## Rust, Axum and Tokio

Rust provides memory safety and predictable performance for long-running infrastructure operations. Axum supplies the HTTP and WebSocket server, while Tokio runs asynchronous database, network and terminal work.

## PostgreSQL and SQLx

The project uses PostgreSQL for relational integrity across users, course content, environments, instances, flags and progress. SQLx is used for typed query mapping and transactions.

SQLx compile-time query checking requires the development database or prepared offline metadata to match the schema expected by the repositories.

## Layered backend

The backend is split into routes, middleware, handlers, services and repositories. HTTP concerns remain in handlers, orchestration remains in services and SQL remains in repositories.

## JWT authentication

Login produces a JWT containing identity and role claims. Protected backend routes should validate the Bearer token and derive the acting user from those claims. Browser-side route protection is only a UX layer.

## VirtualBox laboratories

VirtualBox provides isolated, cloneable machines from a scenario template. VM lifecycle calls are blocking system operations and therefore run through Tokio's blocking-task facility.

## Environment and instance separation

An environment represents a user's scenario session and its lifecycle. An instance represents a VM inside that environment. This allows the model to support multiple instances per environment later, even though the current flow creates one entry-point instance.

## Environment ID for terminal connections

The WebSocket URL uses an environment UUID rather than exposing an SSH port. The backend resolves the running instance and host port, keeping host networking details out of the browser contract.

## WebSocket-to-SSH bridge

xterm.js provides the browser terminal. Axum upgrades the request to a WebSocket and bridges it to an SSH session on the VM. This avoids exposing SSH directly to the browser.

## Scenario-backed Lab tasks

Academy tasks optionally reference scenarios. Only `LAB` tasks are eligible for scenario progress validation. A correct flag is checked against the environment scenario before the task is marked completed.

## Known decisions still required

- Whether a user may run one active environment per scenario or one globally.
- How idle environments are expired and cleaned up.
- Whether VM provisioning should become an asynchronous job with progress polling.
- How scenario and VM-template administration will be exposed safely.
