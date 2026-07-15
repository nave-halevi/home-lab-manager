import AdminEmptyState from "../components/AdminEmptyState";
import AdminHeader from "../components/AdminHeader";
import AdminTable from "../components/AdminTable";
import PaginationControls from "../components/PaginationControls";
import useAdminActivity from "../hooks/useAdminActivity";
import useAdminListQuery from "../hooks/useAdminListQuery";
import { Page } from "./AdminOverviewPage";

const filterKeys = ["action", "entity_type", "order"];

export default function AdminActivityPage() {
  const { params, setPage, setFilter } = useAdminListQuery(filterKeys);
  const query = { ...params, order: params.order || "desc" };
  const { items, meta, loading, error } = useAdminActivity(params);

  return (
    <Page>
      <AdminHeader title="Admin Activity" description="Audit trail of important Admin operations." />

      <div className="mb-5 grid gap-3 md:grid-cols-3">
        <input
          value={params.action}
          onChange={(event) => setFilter("action", event.target.value)}
          placeholder="Exact action"
          className="rounded-lg border border-zinc-700 bg-zinc-900 px-4 py-2.5"
        />
        <input
          value={params.entity_type}
          onChange={(event) => setFilter("entity_type", event.target.value)}
          placeholder="Entity type"
          className="rounded-lg border border-zinc-700 bg-zinc-900 px-4 py-2.5"
        />
        <select
          value={query.order}
          onChange={(event) => setFilter("order", event.target.value)}
          className="rounded-lg border border-zinc-700 bg-zinc-900 px-4 py-2.5"
        >
          <option value="desc">Newest first</option>
          <option value="asc">Oldest first</option>
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
          <AdminTable headers={["Admin", "Action", "Entity", "Entity ID", "Time", "Details"]}>
            {items.map((item) => (
              <tr key={item.id}>
                <td className="px-4 py-3">
                  {item.admin_user_name || "Unknown"}
                  <div className="text-xs text-zinc-500">{item.admin_email}</div>
                </td>
                <td className="px-4 py-3">{item.action}</td>
                <td className="px-4 py-3">{item.entity_type}</td>
                <td className="px-4 py-3 font-mono text-xs text-zinc-500">{item.entity_id || "—"}</td>
                <td className="px-4 py-3 text-zinc-400">{new Date(item.created_at).toLocaleString()}</td>
                <td className="max-w-md px-4 py-3">
                  <pre className="whitespace-pre-wrap break-words rounded-lg bg-zinc-900 p-2 text-xs text-zinc-400">
                    {item.details ? JSON.stringify(item.details, null, 2) : "—"}
                  </pre>
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
