# API

All backend endpoints are exposed under the `/api` prefix.

Authentication

POST /api/auth/register

POST /api/auth/login

Users

GET /api/users

Academy

GET /api/academy/courses

GET /api/academy/courses/:id/full

Admin (Academy)

POST /api/academy/admin/courses

PUT /api/academy/admin/courses/:id

DELETE /api/academy/admin/courses/:id

POST /api/academy/admin/sections

GET /api/academy/admin/sections/:id

PUT /api/academy/admin/sections/:id

DELETE /api/academy/admin/sections/:id

GET /api/academy/admin/courses/:course_id/sections

POST /api/academy/admin/tasks

GET /api/academy/admin/tasks/:id

PUT /api/academy/admin/tasks/:id

DELETE /api/academy/admin/tasks/:id

GET /api/academy/admin/sections/:section_id/tasks

Lab Engine

POST /api/lab/create

POST /api/lab/delete

POST /api/lab/submit

GET /api/lab/terminal/:port

Notes

- `/api/lab/terminal/:port` is used for WebSocket terminal connections.
- `/api/users` is protected by admin middleware.
- Some admin routes are under `/api/academy/admin` and require authentication.