export default function AdminTable({ headers, children }) {
  return <div className="overflow-x-auto rounded-xl border border-zinc-800"><table className="min-w-full divide-y divide-zinc-800 text-left text-sm"><thead className="bg-zinc-900 text-zinc-400"><tr>{headers.map((h) => <th key={h} className="px-4 py-3 font-medium">{h}</th>)}</tr></thead><tbody className="divide-y divide-zinc-800 bg-zinc-950">{children}</tbody></table></div>;
}
