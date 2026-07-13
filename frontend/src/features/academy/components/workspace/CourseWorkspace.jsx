import TaskRenderer from "./TaskRenderer";

export default function CourseWorkspace({ task, onTaskCompleted }) {
  return (
    <div className="flex gap-6 w-full h-full min-h-0">
      <TaskRenderer task={task} onTaskCompleted={onTaskCompleted} />
    </div>
  );
}
