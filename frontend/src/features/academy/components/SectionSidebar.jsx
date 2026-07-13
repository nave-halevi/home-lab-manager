import SectionItem from "./SectionItem";

export default function SectionSidebar({
  sections,
  selectedTask,
  onSelectTask,
  progressByTaskId,
}) {
  return (
    <aside
      className="
        w-72
        shrink-0
        border-r
        border-zinc-800
        pr-6
      "
    >
      <h2 className="text-xl font-bold text-white mb-5">Course Content</h2>

      <div className="space-y-5">
        {sections.map((section) => (
          <SectionItem
            key={section.id}
            section={section}
            selectedTask={selectedTask}
            progressByTaskId={progressByTaskId}
            onSelectTask={onSelectTask}
          />
        ))}
      </div>
    </aside>
  );
}
