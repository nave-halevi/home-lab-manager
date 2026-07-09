import LessonLayout from "./layouts/LessonLayout";
import PracticeLayout from "./layouts/PracticeLayout";
import LabLayout from "./layouts/LabLayout";

export default function TaskRenderer({ task }) {
  if (!task) {
    return (
      <div className="flex h-full items-center justify-center text-zinc-500">
        Select a task from the sidebar.
      </div>
    );
  }

  switch (task.task_type) {
    case "LESSON":
      return <LessonLayout task={task} />;

    case "PRACTICE":
      return <PracticeLayout task={task} />;

    case "LAB":
      return <LabLayout task={task} />;

    default:
      return <LessonLayout task={task} />;
  }
}
