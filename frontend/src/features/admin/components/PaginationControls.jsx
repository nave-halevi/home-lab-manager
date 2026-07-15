export default function PaginationControls({ meta, onPageChange }) {
  if (!meta || meta.total_pages <= 1) return null;

  return (
    <div className="mt-5 flex flex-wrap items-center justify-between gap-3 text-sm text-zinc-400">
      <span>
        Page {meta.page} of {meta.total_pages} · {meta.total_items} results
      </span>
      <div className="flex gap-2">
        <button
          type="button"
          disabled={meta.page <= 1}
          onClick={() => onPageChange(meta.page - 1)}
          className="rounded-lg border border-zinc-700 px-3 py-2 disabled:cursor-not-allowed disabled:opacity-50"
        >
          Previous
        </button>
        <button
          type="button"
          disabled={meta.page >= meta.total_pages}
          onClick={() => onPageChange(meta.page + 1)}
          className="rounded-lg border border-zinc-700 px-3 py-2 disabled:cursor-not-allowed disabled:opacity-50"
        >
          Next
        </button>
      </div>
    </div>
  );
}
