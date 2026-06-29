import { NavLink, useNavigate } from "react-router-dom";
import { useAuth } from "../../context/AuthContext";

export default function AppNavbar() {
  const { logout, user } = useAuth();
  const navigate = useNavigate();

  const linkClass = ({ isActive }) =>
    isActive
      ? "text-white font-medium"
      : "text-zinc-400 hover:text-white transition";

  const handleLogout = () => {
    logout();
    navigate("/login");
  };

  return (
    <header className="border-b border-zinc-800 bg-zinc-950">
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
            className="text-zinc-400 hover:text-white transition"
          >
            {user?.email || "Profile"}
          </NavLink>

          {user?.role === "admin" && (
            <NavLink
              to="/admin"
              className="text-red-400 hover:text-red-300 transition"
            >
              Admin
            </NavLink>
          )}

          <button
            onClick={handleLogout}
            className="text-red-400 hover:text-red-300 transition"
          >
            Logout
          </button>
        </div>
      </div>
    </header>
  );
}
