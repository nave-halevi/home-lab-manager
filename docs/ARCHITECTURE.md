# System Architecture

Frontend

↓

REST API

↓

Axum

↓

Handlers

↓

Services

↓

Repositories

↓

PostgreSQL

↓

VirtualBox Integration

---

Each HTTP request follows the same pipeline:

Route

↓

Handler

↓

Service

↓

Repository

↓

Database