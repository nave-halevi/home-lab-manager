export default function ConfirmActionModal({ open, title, message, busy, onConfirm, onCancel }) {
  if (!open) return null;
  return <div className="fixed inset-0 z-[100] flex items-center justify-center bg-black/70 p-4"><div className="w-full max-w-md rounded-xl border border-zinc-700 bg-zinc-900 p-6"><h2 className="text-xl font-semibold">{title}</h2><p className="mt-3 text-sm text-zinc-400">{message}</p><div className="mt-6 flex justify-end gap-3"><button onClick={onCancel} disabled={busy} className="rounded-lg border border-zinc-700 px-4 py-2 text-sm">Cancel</button><button onClick={onConfirm} disabled={busy} className="rounded-lg bg-red-600 px-4 py-2 text-sm font-semibold">{busy ? "Working..." : "Confirm"}</button></div></div></div>;
}
