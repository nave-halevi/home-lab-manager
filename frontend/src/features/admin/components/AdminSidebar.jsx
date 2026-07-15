import { NavLink } from "react-router-dom";

const items = [
  ["Overview", "/admin"], ["Academy", "/admin/academy"],
  ["Scenarios", "/admin/scenarios"], ["Users", "/admin/users"],
  ["Labs", "/admin/labs"], ["Flags", "/admin/flags"],
  ["Activity", "/admin/activity"],
];

export default function AdminSidebar() {
  return (
    <aside className="border-b border-zinc-800 bg-zinc-900 lg:w-60 lg:shrink-0 lg:border-b-0 lg:border-r">
      <div className="p-5"><p className="text-xs font-semibold uppercase tracking-widest text-red-500">Administration</p></div>
      <nav className="flex gap-2 overflow-x-auto px-3 pb-4 lg:flex-col">
        {items.map(([label, to]) => (
          <NavLink key={to} to={to} end={to === "/admin"} className={({ isActive }) => `whitespace-nowrap rounded-lg px-4 py-2.5 text-sm transition ${isActive ? "bg-red-600 text-white" : "text-zinc-400 hover:bg-zinc-800 hover:text-white"}`}>
            {label}
          </NavLink>
        ))}
      </nav>
      <div className="hidden border-t border-zinc-800 p-4 lg:block"><NavLink to="/dashboard" className="text-sm text-zinc-400 hover:text-white">← Back to Application</NavLink></div>
    </aside>
  );
}
