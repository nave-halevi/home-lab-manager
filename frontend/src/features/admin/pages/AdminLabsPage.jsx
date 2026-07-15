import { useState } from "react";

import AdminEmptyState from "../components/AdminEmptyState";
import AdminHeader from "../components/AdminHeader";
import AdminTable from "../components/AdminTable";
import ConfirmActionModal from "../components/ConfirmActionModal";
import PaginationControls from "../components/PaginationControls";
import useAdminListQuery from "../hooks/useAdminListQuery";
import useAdminLabs from "../hooks/useAdminLabs";
import { terminateAdminLab } from "../services/adminService";
import { Page } from "./AdminOverviewPage";

const terminableStatuses = new Set(["Building", "Running", "Failed"]);
const filterKeys = ["search", "status"];

export default function AdminLabsPage() {
  const [terminating, setTerminating] = useState(null);
  const [busyId, setBusyId] = useState(null);
  const [actionError, setActionError] = useState(null);
  const { params, setPage, setFilter } = useAdminListQuery(filterKeys);
  const { items, meta, loading, error, reload } = useAdminLabs(params);

  const terminate = async () => {
    setBusyId(terminating.environment_id);
    setActionError(null);
    try {
      await terminateAdminLab(terminating.environment_id);
      setTerminating(null);
      await reload();
    } catch (requestError) {
      setActionError(requestError.message);
    } finally {
      setBusyId(null);
    }
  };

  return (
    <Page>
      <AdminHeader title="Labs" description="Active and recent environment state. No VM credentials are exposed." />

      <div className="mb-5 grid gap-3 md:grid-cols-[1fr_220px]">
        <input
          value={params.search}
          onChange={(event) => setFilter("search", event.target.value)}
          placeholder="Search user, email, or scenario"
          className="rounded-lg border border-zinc-700 bg-zinc-900 px-4 py-2.5"
        />
        <select
          value={params.status}
          onChange={(event) => setFilter("status", event.target.value)}
          className="rounded-lg border border-zinc-700 bg-zinc-900 px-4 py-2.5"
        >
          <option value="">All statuses</option>
          {["Building", "Running", "Stopping", "Failed", "Destroyed"].map((item) => (
            <option key={item} value={item.toLowerCase()}>
              {item}
            </option>
          ))}
        </select>
      </div>

      {actionError && <p className="mb-4 text-red-400">{actionError}</p>}
      {loading ? (
        <p>Loading...</p>
      ) : error ? (
        <p className="text-red-400">{error}</p>
      ) : !items.length ? (
        <AdminEmptyState />
      ) : (
        <>
          <AdminTable headers={["User", "Scenario", "Environment", "Instance", "VM", "Created", "Actions"]}>
            {items.map((lab) => (
              <tr key={lab.environment_id}>
                <td className="px-4 py-3">
                  {lab.user_name}
                  <div className="text-xs text-zinc-500">{lab.email}</div>
                </td>
                <td className="px-4 py-3">{lab.scenario_title}</td>
                <td className="px-4 py-3">{lab.environment_status}</td>
                <td className="px-4 py-3">{lab.instance_status || "—"}</td>
                <td className="px-4 py-3 font-mono text-xs text-zinc-400">{lab.vm_name || "—"}</td>
                <td className="px-4 py-3 text-zinc-400">{new Date(lab.created_at).toLocaleString()}</td>
                <td className="px-4 py-3">
                  {terminableStatuses.has(lab.environment_status) ? (
                    <button
                      disabled={busyId === lab.environment_id}
                      onClick={() => setTerminating(lab)}
                      className="text-sm text-red-400 disabled:opacity-50"
                    >
                      {busyId === lab.environment_id ? "Terminating..." : "Terminate"}
                    </button>
                  ) : (
                    <span className="text-sm text-zinc-600">—</span>
                  )}
                </td>
              </tr>
            ))}
          </AdminTable>
          <PaginationControls meta={meta} onPageChange={setPage} />
        </>
      )}

      <ConfirmActionModal
        open={!!terminating}
        title="Terminate lab?"
        message="The VM will be deleted and the Environment and Instance records will be marked Destroyed."
        busy={!!busyId}
        onCancel={() => setTerminating(null)}
        onConfirm={terminate}
      />
    </Page>
  );
}
