import { useState } from "react";

export default function SectionItem({ section, selectedTask, onSelectTask }) {
  const [open, setOpen] = useState(true);

  return (
    <div>
      <button
        onClick={() => setOpen(!open)}
        className="
          w-full
          flex
          items-center
          gap-2
          text-left
          text-zinc-200
          font-semibold
          hover:text-white
          transition
        "
      >
        <span className="w-4 text-red-500 select-none">{open ? "▼" : "▶"}</span>

        <span>{section.title}</span>
      </button>

      {open && (
        <div className="mt-3 ml-6 space-y-1">
          {section.tasks.map((task) => {
            const isActive = selectedTask?.id === task.id;

            return (
              <button
                key={task.id}
                onClick={() => onSelectTask(task)}
                className={`
                  w-full
                  flex
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
                    isActive
                      ? "bg-zinc-900 text-white"
                      : "text-zinc-400 hover:bg-zinc-900/50 hover:text-white"
                  }
                `}
              >
                <span className="w-4 text-center text-red-500 select-none">
                  {isActive ? "●" : "○"}
                </span>

                <span>{task.title}</span>
              </button>
            );
          })}
        </div>
      )}
    </div>
  );
}
