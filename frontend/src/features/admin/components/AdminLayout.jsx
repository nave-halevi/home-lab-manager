import { Outlet } from "react-router-dom";
import AdminSidebar from "./AdminSidebar";

export default function AdminLayout() {
  return <div className="flex min-h-full flex-col bg-zinc-950 lg:flex-row"><AdminSidebar /><div className="min-w-0 flex-1"><Outlet /></div></div>;
}
