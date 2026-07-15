/* eslint-disable react-hooks/set-state-in-effect */
import { useEffect, useState } from "react";

const empty = {
  title: "",
  slug: "",
  description: "",
  difficulty: "",
  is_published: false,
};

export default function CourseForm({
  course,
  onSave,
  onCancel,
  busy,
  error,
}) {
  const [form, setForm] = useState(course ? { ...empty, ...course } : empty);

  useEffect(() => {
    setForm(course ? { ...empty, ...course } : empty);
  }, [course]);

  const change = (event) => {
    setForm((value) => ({
      ...value,
      [event.target.name]:
        event.target.type === "checkbox"
          ? event.target.checked
          : event.target.value,
    }));
  };

  return (
    <form
      onSubmit={(event) => {
        event.preventDefault();
        onSave(form);
      }}
      className="space-y-4 rounded-xl border border-zinc-800 bg-zinc-900 p-5"
    >
      {error && <p className="text-sm text-red-400">{error}</p>}

      <div className="grid gap-4 md:grid-cols-2">
        <Field label="Title" name="title" value={form.title} onChange={change} />
        <Field label="Slug" name="slug" value={form.slug} onChange={change} />
        <Field
          label="Difficulty"
          name="difficulty"
          value={form.difficulty || ""}
          onChange={change}
        />
      </div>

      <label className="block text-sm text-zinc-300">
        Description
        <textarea
          name="description"
          value={form.description || ""}
          onChange={change}
          className="mt-2 min-h-24 w-full rounded-lg border border-zinc-700 bg-zinc-950 p-3"
        />
      </label>

      <label className="flex items-center gap-2 text-sm">
        <input
          type="checkbox"
          name="is_published"
          checked={form.is_published}
          onChange={change}
        />
        Published
      </label>

      <div className="flex gap-3">
        <button disabled={busy} className="rounded-lg bg-red-600 px-4 py-2 font-semibold">
          Save
        </button>
        <button
          type="button"
          onClick={onCancel}
          className="rounded-lg border border-zinc-700 px-4 py-2"
        >
          Cancel
        </button>
      </div>
    </form>
  );
}

function Field({ label, ...props }) {
  return (
    <label className="text-sm text-zinc-300">
      {label}
      <input
        required
        {...props}
        className="mt-2 w-full rounded-lg border border-zinc-700 bg-zinc-950 px-3 py-2"
      />
    </label>
  );
}
