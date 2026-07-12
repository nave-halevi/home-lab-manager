import TerminalWrapper from "../../../../ctf/components/TerminalWrapper";

export default function TerminalWidget({ activeLab }) {
  if (!activeLab) {
    return (
      <div className="flex min-h-[400px] items-center justify-center rounded-xl border border-zinc-800 bg-zinc-950 text-sm text-zinc-500">
        Start the machine to open the terminal.
      </div>
    );
  }

  return (
    <div className="h-full min-h-0 overflow-hidden rounded-xl border border-zinc-800 bg-zinc-950">
      <TerminalWrapper activeLab={activeLab} />
    </div>
  );
}
