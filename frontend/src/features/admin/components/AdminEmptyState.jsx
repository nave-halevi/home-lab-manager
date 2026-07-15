export default function AdminEmptyState({ title = "No records found", description }) {
  return <div className="rounded-xl border border-dashed border-zinc-700 bg-zinc-900/50 p-10 text-center"><h3 className="font-semibold">{title}</h3>{description && <p className="mt-2 text-sm text-zinc-400">{description}</p>}</div>;
}
