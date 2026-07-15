import { useState } from "react";

import AdminEmptyState from "../components/AdminEmptyState";
import AdminHeader from "../components/AdminHeader";
import AdminTable from "../components/AdminTable";
import ConfirmActionModal from "../components/ConfirmActionModal";
import FlagForm from "../components/FlagForm";
import PaginationControls from "../components/PaginationControls";
import useAdminFlags from "../hooks/useAdminFlags";
import useAdminListQuery from "../hooks/useAdminListQuery";
import useAdminScenarios from "../hooks/useAdminScenarios";
import {
  createAdminFlag,
  deleteAdminFlag,
  getAdminFlag,
  updateAdminFlag,
} from "../services/adminService";
import { Page } from "./AdminOverviewPage";

const filterKeys = ["search", "scenario_id"];

export default function AdminFlagsPage() {
  const [editing, setEditing] = useState(undefined);
  const [deleting, setDeleting] = useState(null);
  const [formError, setFormError] = useState(null);
  const [actionError, setActionError] = useState(null);
  const [busy, setBusy] = useState(false);
  const { params, setPage, setFilter } = useAdminListQuery(filterKeys);
  const { items, meta, loading, error, reload } = useAdminFlags(params);
  const { items: scenarios } = useAdminScenarios();

  const openCreate = () => {
    setFormError(null);
    setEditing(null);
  };

  const openEdit = async (flag) => {
    setBusy(true);
    setFormError(null);
    setActionError(null);
    try {
      setEditing(await getAdminFlag(flag.id));
    } catch (requestError) {
      setActionError(requestError.message);
    } finally {
      setBusy(false);
    }
  };

  const closeForm = () => {
    setFormError(null);
    setEditing(undefined);
  };

  const save = async (payload) => {
    setBusy(true);
    setFormError(null);
    try {
      if (editing?.id) {
        await updateAdminFlag(editing.id, payload);
      } else {
        await createAdminFlag(payload);
      }
      closeForm();
      await reload();
    } catch (requestError) {
      setFormError(requestError.message);
    } finally {
      setBusy(false);
    }
  };

  const remove = async () => {
    setBusy(true);
    setActionError(null);
    try {
      await deleteAdminFlag(deleting.id);
      setDeleting(null);
      await reload();
    } catch (requestError) {
      setActionError(requestError.message);
    } finally {
      setBusy(false);
    }
  };

  return (
    <Page>
      <AdminHeader
        title="Flags"
        description="Manage scenario flags. List values stay masked."
        action={
          <button onClick={openCreate} className="rounded-lg bg-red-600 px-4 py-2 font-semibold">
            New Flag
          </button>
        }
      />

      {editing !== undefined && (
        <FlagForm
          flag={editing}
          scenarios={scenarios}
          busy={busy}
          error={formError}
          onCancel={closeForm}
          onSave={save}
        />
      )}

      <div className="mb-5 grid gap-3 md:grid-cols-[1fr_280px]">
        <input
          value={params.search}
          onChange={(event) => setFilter("search", event.target.value)}
          placeholder="Search scenario or flag value"
          className="rounded-lg border border-zinc-700 bg-zinc-900 px-4 py-2.5"
        />
        <select
          value={params.scenario_id}
          onChange={(event) => setFilter("scenario_id", event.target.value)}
          className="rounded-lg border border-zinc-700 bg-zinc-900 px-4 py-2.5"
        >
          <option value="">All scenarios</option>
          {scenarios.map((scenario) => (
            <option key={scenario.id} value={scenario.id}>
              {scenario.title}
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
          <AdminTable headers={["Flag ID", "Scenario", "Value", "Points", "Actions"]}>
            {items.map((flag) => (
              <tr key={flag.id}>
                <td className="px-4 py-3 font-mono text-xs text-zinc-500">{flag.id}</td>
                <td className="px-4 py-3">{flag.scenario_title || "Unassigned"}</td>
                <td className="px-4 py-3 font-mono">{flag.masked_value}</td>
                <td className="px-4 py-3 text-emerald-400">{flag.points}</td>
                <td className="px-4 py-3">
                  <button onClick={() => openEdit(flag)} className="mr-3 text-sm">
                    Edit
                  </button>
                  <button onClick={() => setDeleting(flag)} className="text-sm text-red-400">
                    Delete
                  </button>
                </td>
              </tr>
            ))}
          </AdminTable>
          <PaginationControls meta={meta} onPageChange={setPage} />
        </>
      )}

      <ConfirmActionModal
        open={!!deleting}
        title="Delete flag?"
        message="Solved flags cannot be deleted. This action only succeeds for unsolved flags."
        busy={busy}
        onCancel={() => setDeleting(null)}
        onConfirm={remove}
      />
    </Page>
  );
}
