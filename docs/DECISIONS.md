# Technical Decisions

Rust

Chosen for performance, memory safety and reliability.

Axum

Chosen because it integrates naturally with Tokio and modern async Rust.

SQLx

Chosen for compile-time checked SQL queries.

PostgreSQL

Chosen for relational data integrity and advanced SQL capabilities.

Layered Architecture

The backend is divided into:

Routes

Handlers

Services

Repositories

This separation keeps business logic independent from HTTP and database implementations.

Virtual Machines

VirtualBox is used for isolated practical laboratories.