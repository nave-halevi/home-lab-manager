import { Outlet } from "react-router-dom";
import AppNavbar from "./AppNavbar";

export default function AppLayout() {
  return (
    <div className="min-h-screen bg-zinc-950 text-white flex flex-col">
      <AppNavbar />
      <main className="flex-1 min-h-0 overflow-auto">
        <Outlet />
      </main>
    </div>
  );
}
