# Project Overview

Home Lab Manager is a self-hosted cybersecurity learning platform. It combines structured courses with isolated virtual-machine laboratories and Capture The Flag (CTF) exercises.

## Goals

- Teach Linux, networking and cybersecurity through practical exercises.
- Organize learning content into courses, sections and typed tasks.
- Provision an isolated VirtualBox machine for a lab scenario.
- Expose the machine through an SSH-backed browser terminal.
- Validate flags, award points and record task completion.
- Keep administrative course management separate from the student experience.

## Main domains

### Authentication and users

Users can register and log in. Login returns a JWT and user information. Users have a `user` or `admin` role and a cumulative score.

### Academy

The Academy contains published courses. A course contains ordered sections, and a section contains ordered tasks. Tasks are rendered by type:

- `LESSON`: learning content.
- `PRACTICE`: learning and interaction workspace; still partially implemented.
- `LAB`: scenario-backed machine, terminal and flag submission.

### Lab engine

A lab task may reference a scenario. Starting it creates an environment and instance record, clones a VirtualBox template, starts the VM, waits for SSH and exposes a WebSocket terminal. An active environment can be restored after revisiting the task.

### CTF and progress

Flag submission verifies the user, running environment, task-to-scenario relationship and flag value. A correct flag awards points and marks the associated task as completed. Duplicate flag submissions do not award points twice.

## Technology summary

- Frontend: React, JavaScript/JSX, Vite, Tailwind CSS and xterm.js.
- Backend: Rust, Axum and Tokio.
- Database: PostgreSQL through SQLx.
- Authentication: JWT and bcrypt.
- Lab runtime: VirtualBox, SSH and WebSockets.

## Current maturity

The Academy, authentication and primary Lab flow are implemented. Some UI widgets and the practice experience are placeholders. Lab and Academy admin authorization still require hardening, and the committed database migrations do not yet contain every column used by the current repositories. See [Roadmap](ROADMAP.md) and [Database](DATABASE.md).
