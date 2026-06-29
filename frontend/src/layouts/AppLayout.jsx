import { Outlet, useNavigate } from "react-router-dom";
import AppNavbar from "../shared/layout/AppNavbar";

export default function AppLayout() {
  const navigate = useNavigate();

  const menu = [
    { name: "Dashboard", path: "/dashboard" },
    { name: "Academy", path: "/academy" },
    { name: "Labs", path: "/labs" },
    { name: "Leaderboard", path: "/leaderboard" },
  ];

  return (
    <div className="min-h-screen bg-zinc-950 text-white flex flex-col">
      <AppNavbar />

      <div className="flex flex-1">
        <aside className="w-64 border-r border-zinc-800 bg-zinc-950 p-6">
          <nav className="space-y-3">
            {menu.map((item, i) => (
              <button
                key={i}
                onClick={() => navigate(item.path)}
                className="w-full text-left px-3 py-2 rounded-lg text-zinc-400 hover:text-white hover:bg-zinc-900 transition"
              >
                {item.name}
              </button>
            ))}
          </nav>
        </aside>

        <main className="flex-1 overflow-auto">
          <Outlet />
        </main>
      </div>
    </div>
  );
}
