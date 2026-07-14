import { NavLink } from "react-router-dom";
import { useAuth } from "../context/AuthContext";

export default function AppNavbar() {
  const { user } = useAuth();

  const linkClass = ({ isActive }) =>
    isActive
      ? "text-white font-medium"
      : "text-zinc-400 hover:text-white transition";

  const profileInitial = (user?.user_name || user?.email || "U")
    .charAt(0)
    .toUpperCase();

  return (
    <header className="sticky top-0 z-50 border-b border-zinc-800 bg-zinc-950/90 backdrop-blur">
      <div className="mx-auto flex h-16 max-w-7xl items-center justify-between px-8">
        {/* Left - Logo + Nav */}
        <div className="flex items-center gap-8">
          <h1 className="text-xl font-bold">
            <span className="text-red-600">Cyber</span>Range
          </h1>

          <nav className="flex gap-6 text-sm">
            <NavLink to="/dashboard" className={linkClass}>
              Dashboard
            </NavLink>

            <NavLink to="/academy" className={linkClass}>
              Academy
            </NavLink>

            <NavLink to="/machines" className={linkClass}>
              Machines
            </NavLink>

            <NavLink to="/leaderboard" className={linkClass}>
              Leaderboard
            </NavLink>
          </nav>
        </div>

        {/* Right - User section */}
        <div className="flex items-center gap-4 text-sm">
          <NavLink
            to="/profile"
            aria-label="Open profile"
            className="flex items-center gap-3 text-zinc-400 transition hover:text-white"
          >
            <span className="hidden sm:inline">{user?.email || "Profile"}</span>

            <span className="flex h-9 w-9 items-center justify-center overflow-hidden rounded-full border border-zinc-700 bg-zinc-900 text-sm font-semibold text-white">
              {user?.avatar_url ? (
                <img
                  src={user.avatar_url}
                  alt=""
                  className="h-full w-full object-cover"
                />
              ) : (
                profileInitial
              )}
            </span>
          </NavLink>

          {user?.role?.toLowerCase() === "admin" && (
            <NavLink
              to="/admin"
              className="text-red-400 hover:text-red-300 transition"
            >
              Admin
            </NavLink>
          )}
        </div>
      </div>
    </header>
  );
}
