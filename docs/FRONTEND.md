# Frontend

## Stack

- React 19.
- JavaScript and JSX, not TypeScript.
- Vite 8.
- React Router.
- Tailwind CSS.
- Browser Fetch API.
- xterm.js with the fit addon.

## Routes

Public routes:

- `/`: landing page.
- `/login`: login form.
- `/register`: registration form.

Routes wrapped by `RequireAuth`:

- `/dashboard`: user dashboard.
- `/academy`: published course catalog.
- `/academy/:courseId`: course and task workspace.
- `/machines`: standalone machine-selection prototype.
- `/leaderboard`: placeholder.

The router has no catch-all 404 route at present.

## Feature structure

### Authentication

`AuthContext` stores the current user and JWT, restores both from `localStorage` and exposes login, registration and logout functions. API clients add the stored token as a Bearer header where implemented.

`RequireAuth` protects browser navigation. Backend middleware remains responsible for actual data security.

### Academy catalog

The Academy loads a published course list and navigates to a full course. The course page displays ordered sections and tasks, selects the first available task by default and renders the selected task in a workspace.

### Task renderer

`TaskRenderer` currently maps:

- `LESSON` to `LessonLayout`.
- `PRACTICE` to `PracticeLayout`.
- `LAB` to `LabLayout`.
- Unknown types to the lesson layout.

`LESSON` displays textual content. `LAB` displays content, machine controls, an embedded terminal and flag submission. `PRACTICE` has a two-panel layout but its terminal does not currently receive an active Lab, so it remains partial.

The `VideoWidget`, `DownloadWidget` and `HintWidget` are placeholders returning no UI.

### Lab state

`useLabs` manages:

- active environment state;
- loading and error state;
- active-environment restoration by scenario;
- environment creation;
- environment deletion.

When a Lab task is selected, `LabLayout` requests the active environment for its scenario. This lets the UI reconnect after a refresh or after leaving and reopening the task. If creation reports that an environment already exists, the hook also attempts restoration.

The state is local to each hook instance; there is no application-wide Lab context.

### Terminal

`TerminalWrapper` creates an xterm.js instance and connects a WebSocket to:

```text
ws(s)://<api-origin>/api/lab/terminal/<environment-id>
```

Terminal input is sent to the socket and received data is written to xterm. Resize events refit the terminal. The socket and terminal are disposed when the component unmounts.

### Flags

`FlagWidget` requires an environment ID and task ID. It submits a normalized flag, shows feedback and disables itself after a correct answer. The server is the source of truth for validation, scoring and completion.

## API configuration

Frontend clients use `VITE_API_URL` as the API origin. If it is absent, they fall back to `http://localhost:3000`. The WebSocket origin is derived by replacing `http` with `ws`.

## Shared UI

Reusable components currently include `Button`, `Card` and `Input`. Academy-specific panels and widgets live under the Academy workspace rather than the shared UI directory.

## Legacy and prototype code

The repository contains Lab components and pages that are not routed by `App.jsx`, including `LabsPage`, `LabWorkspace`, `LabList` and `CreateLabButton`. The `/machines` page also uses hard-coded scenarios and should be treated as a prototype rather than the main Academy Lab flow.

## Known frontend issues

- Some widgets and the Practice interaction are incomplete.
- The leaderboard is a placeholder; profile and admin pages do not exist.
- There is no 404 route.
- Lab state is not global across unrelated pages.
- The standalone Machines page uses hard-coded scenario UUIDs.
- Some components contain development `console.log` statements.
