import { useMemo, useState } from "react";

export default function SectionItem({
  section,
  selectedTask,
  progressByTaskId,
  onSelectTask,
}) {
  const [open, setOpen] = useState(true);

  const sectionProgress = useMemo(() => {
    const tasks = section.tasks || [];

    const totalTasks = tasks.length;

    const completedTasks = tasks.filter((task) => {
      const taskProgress = progressByTaskId?.get(task.id);

      return taskProgress?.progress_status === "COMPLETED";
    }).length;

    const isCompleted = totalTasks > 0 && completedTasks === totalTasks;

    return {
      totalTasks,
      completedTasks,
      isCompleted,
    };
  }, [section.tasks, progressByTaskId]);

  return (
    <div>
      <button
        type="button"
        onClick={() => setOpen((current) => !current)}
        className="
          flex
          w-full
          items-center
          gap-2
          text-left
          font-semibold
          text-zinc-200
          transition
          hover:text-white
        "
      >
        <span className="w-4 select-none text-red-500">{open ? "▼" : "▶"}</span>

        <span className="min-w-0 flex-1 truncate">{section.title}</span>

        <span
          className={`
            shrink-0
            text-xs
            font-medium
            ${
              sectionProgress.isCompleted ? "text-emerald-400" : "text-zinc-500"
            }
          `}
        >
          {sectionProgress.isCompleted && "✓ "}
          {sectionProgress.completedTasks}/{sectionProgress.totalTasks}
        </span>
      </button>

      {open && (
        <div className="ml-6 mt-3 space-y-1">
          {(section.tasks || []).map((task) => {
            const taskProgress = progressByTaskId?.get(task.id);

            const progressStatus =
              taskProgress?.progress_status ?? "NOT_STARTED";

            const accessStatus = taskProgress?.access_status ?? "LOCKED";

            const isCompleted = progressStatus === "COMPLETED";

            const isLocked = accessStatus === "LOCKED";

            const isActive = selectedTask?.id === task.id;

            const handleTaskClick = () => {
              if (isLocked) {
                return;
              }

              onSelectTask(task);
            };

            return (
              <button
                type="button"
                key={task.id}
                onClick={handleTaskClick}
                disabled={isLocked}
                title={
                  isLocked
                    ? "Complete the previous task to unlock this task."
                    : task.title
                }
                className={`
                  flex
                  w-full
                  items-center
                  gap-2
                  rounded-lg
                  px-2
                  py-1.5
                  text-left
                  text-sm
                  transition-all
                  duration-200
                  ${
                    isLocked
                      ? "cursor-not-allowed text-zinc-600 opacity-60"
                      : isActive
                        ? "bg-zinc-900 text-white"
                        : "text-zinc-400 hover:bg-zinc-900/50 hover:text-white"
                  }
                `}
              >
                <span
                  className={`
                    w-4
                    select-none
                    text-center
                    ${
                      isCompleted
                        ? "text-emerald-400"
                        : isLocked
                          ? "text-zinc-600"
                          : isActive
                            ? "text-red-500"
                            : "text-zinc-500"
                    }
                  `}
                >
                  {isCompleted ? "✓" : isLocked ? "🔒" : isActive ? "●" : "○"}
                </span>

                <span
                  className={`
                    min-w-0
                    flex-1
                    truncate
                    ${isCompleted ? "text-emerald-300" : ""}
                  `}
                >
                  {task.title}
                </span>

                {isCompleted && (
                  <span className="text-[10px] font-medium uppercase tracking-wide text-emerald-400">
                    Completed
                  </span>
                )}
              </button>
            );
          })}
        </div>
      )}
    </div>
  );
}
