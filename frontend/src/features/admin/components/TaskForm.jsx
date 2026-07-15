import { useMemo, useState } from "react";

export default function TaskForm({
  task,
  scenarios,
  onSave,
  onCancel,
  busy,
  error,
}) {
  const [form, setForm] = useState({
    title: task?.title || "",
    content: task?.content || "",
    task_type: task?.task_type || "LESSON",
    order_index: task?.order_index ?? 1,
    points: task?.points ?? 10,
    scenario_id: task?.scenario_id || "",
  });

  const scenarioOptions = useMemo(() => {
    const activeScenarios = scenarios.filter((scenario) => scenario.is_active);
    const currentScenario = scenarios.find(
      (scenario) => scenario.id === task?.scenario_id,
    );

    if (currentScenario && !currentScenario.is_active) {
      return [currentScenario, ...activeScenarios];
    }

    return activeScenarios;
  }, [scenarios, task?.scenario_id]);

  const handleChange = (event) => {
    setForm((current) => ({
      ...current,
      [event.target.name]: event.target.value,
    }));
  };

  const handleSubmit = (event) => {
    event.preventDefault();
    onSave({
      ...form,
      order_index: Number(form.order_index),
      points: Number(form.points),
      scenario_id:
        form.task_type === "LAB" ? form.scenario_id || null : null,
    });
  };

  return (
    <form
      onSubmit={handleSubmit}
      className="space-y-3 rounded-lg border border-zinc-700 bg-zinc-950 p-4"
    >
      {error && <p className="text-sm text-red-400">{error}</p>}

      <div className="grid gap-3 md:grid-cols-4">
        <input
          required
          name="title"
          placeholder="Task title"
          value={form.title}
          onChange={handleChange}
          className="rounded-lg border border-zinc-700 bg-zinc-900 px-3 py-2"
        />
        <select
          name="task_type"
          value={form.task_type}
          onChange={handleChange}
          className="rounded-lg border border-zinc-700 bg-zinc-900 px-3 py-2"
        >
          <option>LESSON</option>
          <option>PRACTICE</option>
          <option>LAB</option>
        </select>
        <input
          name="order_index"
          type="number"
          min="0"
          value={form.order_index}
          onChange={handleChange}
          className="rounded-lg border border-zinc-700 bg-zinc-900 px-3 py-2"
        />
        <input
          name="points"
          type="number"
          min="0"
          value={form.points}
          onChange={handleChange}
          className="rounded-lg border border-zinc-700 bg-zinc-900 px-3 py-2"
        />
      </div>

      {form.task_type === "LAB" && (
        <select
          required
          name="scenario_id"
          value={form.scenario_id}
          onChange={handleChange}
          className="w-full rounded-lg border border-zinc-700 bg-zinc-900 px-3 py-2"
        >
          <option value="">Select scenario</option>
          {scenarioOptions.map((scenario) => (
            <option
              key={scenario.id}
              value={scenario.id}
              disabled={!scenario.is_active}
            >
              {scenario.title}
              {!scenario.is_active ? " (Inactive)" : ""}
            </option>
          ))}
        </select>
      )}

      <textarea
        required
        name="content"
        placeholder="Task content"
        value={form.content}
        onChange={handleChange}
        className="min-h-32 w-full rounded-lg border border-zinc-700 bg-zinc-900 p-3"
      />

      <div className="flex gap-2">
        <button
          disabled={busy}
          className="rounded-lg bg-red-600 px-3 py-2"
        >
          Save Task
        </button>
        <button type="button" onClick={onCancel} className="px-3">
          Cancel
        </button>
      </div>
    </form>
  );
}
