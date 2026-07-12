export default function MachineWidget({
  activeLab,
  isLoading,
  error,
  onStart,
  onDelete,
}) {
  const isRunning = activeLab?.status === "Running";

  return (
    <div className="shrink-0 border-b border-zinc-800 bg-zinc-900/90">
      <div className="flex min-h-14 items-center justify-between gap-4 px-4 py-2">
        <div className="flex min-w-0 items-center gap-3">
          <span
            className={`h-2.5 w-2.5 shrink-0 rounded-full ${
              isLoading
                ? "bg-amber-500"
                : isRunning
                  ? "bg-emerald-500"
                  : "bg-zinc-600"
            }`}
          />

          <div className="min-w-0">
            <p className="truncate text-sm font-medium text-zinc-100">
              Kali Lab Machine
            </p>

            <p className="truncate text-xs text-zinc-500">
              {isLoading
                ? isRunning
                  ? "Stopping machine..."
                  : "Starting machine..."
                : isRunning
                  ? "Running"
                  : "Stopped"}
            </p>
          </div>
        </div>

        {isRunning ? (
          <button
            type="button"
            disabled={isLoading}
            onClick={onDelete}
            className="shrink-0 rounded-md border border-red-900 bg-red-950/50 px-3 py-1.5 text-xs font-medium text-red-300 transition hover:bg-red-950 disabled:cursor-not-allowed disabled:opacity-50"
          >
            {isLoading ? "Stopping..." : "Terminate"}
          </button>
        ) : (
          <button
            type="button"
            disabled={isLoading}
            onClick={onStart}
            className="shrink-0 rounded-md bg-emerald-600 px-3 py-1.5 text-xs font-medium text-white transition hover:bg-emerald-500 disabled:cursor-not-allowed disabled:opacity-50"
          >
            {isLoading ? "Starting..." : "Start machine"}
          </button>
        )}
      </div>

      {error && (
        <div className="border-t border-red-900 bg-red-950/30 px-4 py-2 text-xs text-red-300">
          {error}
        </div>
      )}
    </div>
  );
}
