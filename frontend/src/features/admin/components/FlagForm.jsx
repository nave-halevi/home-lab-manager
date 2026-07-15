/* eslint-disable react-hooks/set-state-in-effect */
import { useEffect, useState } from "react";

const empty = { scenario_id: "", flag_value: "", points: 10 };

export default function FlagForm({ flag, scenarios, busy, error, onSave, onCancel }) {
  const [form, setForm] = useState(empty);

  useEffect(() => {
    setForm(flag ? { ...empty, ...flag } : empty);
  }, [flag]);

  const change = (event) => {
    setForm((current) => ({ ...current, [event.target.name]: event.target.value }));
  };

  return (
    <form
      onSubmit={(event) => {
        event.preventDefault();
        onSave({ ...form, points: Number(form.points) });
      }}
      className="mb-6 space-y-4 rounded-xl border border-zinc-800 bg-zinc-900 p-5"
    >
      {error && <p className="text-sm text-red-400">{error}</p>}

      <div className="grid gap-4 md:grid-cols-[1fr_1fr_120px]">
        <select
          required
          name="scenario_id"
          value={form.scenario_id || ""}
          onChange={change}
          className="rounded-lg border border-zinc-700 bg-zinc-950 px-3 py-2"
        >
          <option value="">Select scenario</option>
          {scenarios.map((scenario) => (
            <option key={scenario.id} value={scenario.id} disabled={!scenario.is_active && !flag}>
              {scenario.title}
              {!scenario.is_active ? " (Inactive)" : ""}
            </option>
          ))}
        </select>
        <input
          required
          name="flag_value"
          value={form.flag_value || ""}
          onChange={change}
          placeholder="Full flag value"
          className="rounded-lg border border-zinc-700 bg-zinc-950 px-3 py-2 font-mono"
        />
        <input
          required
          min="0"
          type="number"
          name="points"
          value={form.points}
          onChange={change}
          className="rounded-lg border border-zinc-700 bg-zinc-950 px-3 py-2"
        />
      </div>

      <div className="flex gap-3">
        <button disabled={busy} className="rounded-lg bg-red-600 px-4 py-2 font-semibold">
          {busy ? "Saving..." : "Save Flag"}
        </button>
        <button
          type="button"
          disabled={busy}
          onClick={onCancel}
          className="rounded-lg border border-zinc-700 px-4 py-2"
        >
          Cancel
        </button>
      </div>
    </form>
  );
}
