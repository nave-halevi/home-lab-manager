import { useState } from "react";

export default function SectionForm({
  section,
  onSave,
  onCancel,
  busy,
  error,
}) {
  const [form, setForm] = useState({
    title: section?.title || "",
    description: section?.description || "",
    order_index: section?.order_index ?? 1,
  });

  return (
    <form
      onSubmit={(event) => {
        event.preventDefault();
        onSave({ ...form, order_index: Number(form.order_index) });
      }}
      className="grid gap-3 rounded-lg border border-zinc-700 bg-zinc-950 p-4 md:grid-cols-[1fr_120px_auto]"
    >
      {error && <p className="text-sm text-red-400 md:col-span-3">{error}</p>}

      <input
        required
        placeholder="Section title"
        value={form.title}
        onChange={(event) => setForm({ ...form, title: event.target.value })}
        className="rounded-lg border border-zinc-700 bg-zinc-900 px-3 py-2"
      />
      <input
        required
        type="number"
        min="0"
        value={form.order_index}
        onChange={(event) =>
          setForm({ ...form, order_index: event.target.value })
        }
        className="rounded-lg border border-zinc-700 bg-zinc-900 px-3 py-2"
      />
      <div className="flex gap-2">
        <button disabled={busy} className="rounded-lg bg-red-600 px-3 py-2">
          Save
        </button>
        <button type="button" onClick={onCancel} className="px-3">
          Cancel
        </button>
      </div>
      <textarea
        placeholder="Description"
        value={form.description}
        onChange={(event) =>
          setForm({ ...form, description: event.target.value })
        }
        className="rounded-lg border border-zinc-700 bg-zinc-900 p-3 md:col-span-3"
      />
    </form>
  );
}
