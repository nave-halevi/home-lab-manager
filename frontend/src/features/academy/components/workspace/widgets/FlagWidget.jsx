import { useCTF } from "../../../../ctf/hooks/useCTF";

export default function FlagWidget({ environmentId, taskId, onTaskCompleted }) {
  const {
    flagInput,
    setFlagInput,
    feedback,
    isSubmitting,
    isSolved,
    handleSubmit,
    clearFeedback,
  } = useCTF(environmentId, taskId);

  const submitFlag = async () => {
    const solved = await handleSubmit();

    if (solved) {
      await onTaskCompleted?.();
    }
  };

  const handleInputChange = (event) => {
    setFlagInput(event.target.value);

    if (feedback) {
      clearFeedback();
    }
  };

  const handleKeyDown = (event) => {
    if (event.key === "Enter") {
      event.preventDefault();
      submitFlag();
    }
  };

  return (
    <section className="rounded-xl border border-zinc-800 bg-zinc-950/60 p-4">
      <div className="mb-3">
        <h3 className="text-sm font-semibold text-zinc-100">Submit Flag</h3>

        <p className="mt-1 text-xs leading-5 text-zinc-500">
          Find the flag inside the machine and submit its exact value.
        </p>
      </div>

      <div className="flex flex-col gap-3 sm:flex-row">
        <input
          type="text"
          value={flagInput}
          onChange={handleInputChange}
          onKeyDown={handleKeyDown}
          disabled={!environmentId || !taskId || isSubmitting || isSolved}
          placeholder="CTF{...}"
          autoComplete="off"
          spellCheck={false}
          className="min-w-0 flex-1 rounded-lg border border-zinc-800 bg-zinc-900 px-3 py-2 text-sm text-zinc-100 outline-none transition placeholder:text-zinc-600 focus:border-emerald-600 disabled:cursor-not-allowed disabled:opacity-50"
        />

        <button
          type="button"
          onClick={handleSubmit}
          disabled={
            !environmentId ||
            !taskId ||
            !flagInput.trim() ||
            isSubmitting ||
            isSolved
          }
          className="rounded-lg bg-emerald-600 px-4 py-2 text-sm font-medium text-white transition hover:bg-emerald-500 disabled:cursor-not-allowed disabled:opacity-50"
        >
          {isSubmitting ? "Checking..." : isSolved ? "Solved" : "Submit"}
        </button>
      </div>

      {!environmentId && (
        <p className="mt-3 text-xs text-amber-400">
          Start the machine before submitting a flag.
        </p>
      )}

      {environmentId && !taskId && (
        <p className="mt-3 text-xs text-red-400">
          This lab is missing its task identifier.
        </p>
      )}

      {feedback && (
        <div
          className={`mt-3 rounded-lg border p-3 text-sm ${
            isSolved ||
            feedback.includes("✅") ||
            feedback.includes("already submitted")
              ? "border-emerald-900 bg-emerald-950/30 text-emerald-300"
              : feedback.includes("⚠️")
                ? "border-amber-900 bg-amber-950/30 text-amber-300"
                : "border-red-900 bg-red-950/30 text-red-300"
          }`}
        >
          {feedback}
        </div>
      )}
    </section>
  );
}
