import { Outlet } from "react-router-dom";
import PublicNavbar from "../components/layout/PublicNavbar";

export default function PublicLayout() {
  return (
    <div className="min-h-screen bg-zinc-950 text-white">
      <PublicNavbar />

      <main>
        <Outlet />
      </main>
    </div>
  );
}
