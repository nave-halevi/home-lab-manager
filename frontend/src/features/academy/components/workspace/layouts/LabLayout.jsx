import { useEffect, useMemo } from "react";

import { useLabs } from "../../../../labs/hooks/useLabs";

import InteractionPanel from "../panels/InteractionPanel";
import LearningPanel from "../panels/LearningPanel";

import FlagWidget from "../widgets/FlagWidget";
import MachineWidget from "../widgets/MachineWidget";
import TerminalWidget from "../widgets/TerminalWidget";

export default function LabLayout({ task, onTaskCompleted }) {
  const {
    activeLab,
    isLoading,
    error,
    restoreActiveLab,
    handleCreateLab,
    handleDeleteLab,
  } = useLabs();

  const scenarioId = useMemo(() => {
    return (
      task?.scenario_id ||
      task?.scenarioId ||
      task?.content?.scenario_id ||
      null
    );
  }, [task]);

  useEffect(() => {
    if (!scenarioId) {
      return;
    }

    restoreActiveLab(scenarioId);
  }, [scenarioId, restoreActiveLab]);

  return (
    <>
      <LearningPanel>
        <div className="flex min-h-full flex-col gap-5 text-white">
          <div>
            <h2 className="text-xl font-semibold">
              {task?.title || "Laboratory"}
            </h2>

            <div className="mt-3 whitespace-pre-wrap text-sm leading-7 text-zinc-300">
              {typeof task?.content === "string"
                ? task.content
                : task?.description ||
                  "Complete the laboratory using the interactive machine."}
            </div>
          </div>

          {!scenarioId ? (
            <div className="rounded-xl border border-amber-900 bg-amber-950/30 p-4 text-sm text-amber-300">
              This lab task is not connected to a scenario. Add a scenario_id
              before starting a machine.
            </div>
          ) : (
            <FlagWidget
              environmentId={activeLab?.envId}
              taskId={task?.id}
              onTaskCompleted={onTaskCompleted}
            />
          )}
        </div>
      </LearningPanel>

      <InteractionPanel>
        <div className="flex h-full min-h-0 flex-col overflow-hidden rounded-xl border border-zinc-800 bg-zinc-950">
          <MachineWidget
            activeLab={activeLab}
            isLoading={isLoading}
            error={error}
            onStart={() => handleCreateLab(scenarioId)}
            onDelete={handleDeleteLab}
          />

          <div className="min-h-0 flex-1">
            <TerminalWidget activeLab={activeLab} />
          </div>
        </div>
      </InteractionPanel>
    </>
  );
}
