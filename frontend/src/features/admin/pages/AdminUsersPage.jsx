import { Link } from "react-router-dom";

import AdminEmptyState from "../components/AdminEmptyState";
import AdminHeader from "../components/AdminHeader";
import AdminTable from "../components/AdminTable";
import PaginationControls from "../components/PaginationControls";
import useAdminListQuery from "../hooks/useAdminListQuery";
import useAdminUsers from "../hooks/useAdminUsers";
import { Page } from "./AdminOverviewPage";

const date = (value) => new Date(value).toLocaleDateString();
const filterKeys = ["search", "status"];

export default function AdminUsersPage() {
  const { params, setPage, setFilter } = useAdminListQuery(filterKeys);
  const { items, meta, loading, error } = useAdminUsers(params);

  return (
    <Page>
      <AdminHeader title="Users" description="Inspect registered users and account status." />

      <div className="mb-5 grid gap-3 md:grid-cols-[1fr_220px]">
        <input
          value={params.search}
          onChange={(event) => setFilter("search", event.target.value)}
          placeholder="Search username or email"
          className="rounded-lg border border-zinc-700 bg-zinc-900 px-4 py-2.5"
        />
        <select
          value={params.status}
          onChange={(event) => setFilter("status", event.target.value)}
          className="rounded-lg border border-zinc-700 bg-zinc-900 px-4 py-2.5"
        >
          <option value="">All users</option>
          <option value="active">Active</option>
          <option value="disabled">Disabled</option>
          <option value="admin">Admins</option>
          <option value="user">Users</option>
        </select>
      </div>

      {loading ? (
        <p>Loading...</p>
      ) : error ? (
        <p className="text-red-400">{error}</p>
      ) : !items.length ? (
        <AdminEmptyState />
      ) : (
        <>
          <AdminTable headers={["User", "Email", "Role", "Status", "Score", "Joined", "Actions"]}>
            {items.map((user) => (
              <tr key={user.id}>
                <td className="px-4 py-3">
                  <div className="flex items-center gap-3">
                    <span className="flex h-9 w-9 items-center justify-center overflow-hidden rounded-full bg-zinc-800">
                      {user.avatar_url ? (
                        <img src={user.avatar_url} alt="" className="h-full w-full object-cover" />
                      ) : (
                        user.user_name[0]?.toUpperCase()
                      )}
                    </span>
                    {user.user_name}
                  </div>
                </td>
                <td className="px-4 py-3 text-zinc-400">{user.email}</td>
                <td className="px-4 py-3">{user.role}</td>
                <td className={user.is_active ? "px-4 py-3 text-emerald-400" : "px-4 py-3 text-red-400"}>
                  {user.is_active ? "Active" : "Disabled"}
                </td>
                <td className="px-4 py-3 text-emerald-400">{user.total_score}</td>
                <td className="px-4 py-3 text-zinc-400">{date(user.created_at)}</td>
                <td className="px-4 py-3">
                  <Link to={`/admin/users/${user.id}`} className="text-sm hover:text-red-400">
                    Details
                  </Link>
                </td>
              </tr>
            ))}
          </AdminTable>
          <PaginationControls meta={meta} onPageChange={setPage} />
        </>
      )}
    </Page>
  );
}
