export const TaskUiState = {
  LOCKED: "LOCKED",
  AVAILABLE: "AVAILABLE",
  CURRENT: "CURRENT",
  COMPLETED: "COMPLETED",
};

export function getTaskUiState({
  task,
  selectedTask,
  progressByTaskId,
  previousTaskCompleted,
}) {
  const progress = progressByTaskId?.get(task.id);

  if (progress?.status === "COMPLETED") {
    return TaskUiState.COMPLETED;
  }

  if (selectedTask?.id === task.id) {
    return TaskUiState.CURRENT;
  }

  if (!previousTaskCompleted) {
    return TaskUiState.LOCKED;
  }

  return TaskUiState.AVAILABLE;
}
