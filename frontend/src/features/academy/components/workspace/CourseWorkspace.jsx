import TaskRenderer from "./TaskRenderer";

export default function CourseWorkspace({ task }) {
  return (
    <div className="flex gap-6 w-full h-full min-h-0">
      <TaskRenderer task={task} />
    </div>
  );
}
