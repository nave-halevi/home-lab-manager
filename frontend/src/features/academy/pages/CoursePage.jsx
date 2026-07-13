import { useEffect, useMemo, useState } from "react";
import { useParams } from "react-router-dom";

import useCourse from "../hooks/useCourse";
import useCourseProgress from "../hooks/useCourseProgress";

import CourseHeader from "../components/CourseHeader";
import SectionSidebar from "../components/SectionSidebar";
import CourseWorkspace from "../components/workspace/CourseWorkspace";

export default function CoursePage() {
  const { courseId } = useParams();

  const { course, loading, error } = useCourse(courseId);

  const { progress, progressLoading, progressError, reloadProgress } =
    useCourseProgress(courseId);

  const [selectedTask, setSelectedTask] = useState(null);

  const progressByTaskId = useMemo(() => {
    return new Map(
      (progress?.tasks || []).map((taskProgress) => [
        taskProgress.task_id,
        taskProgress,
      ]),
    );
  }, [progress]);

  const getOrderedTasks = () => {
    return course?.sections?.flatMap((section) => section.tasks || []) || [];
  };

  const findNextAvailableTask = (currentProgress) => {
    const orderedTasks = getOrderedTasks();

    const currentProgressByTaskId = new Map(
      (currentProgress?.tasks || []).map((taskProgress) => [
        taskProgress.task_id,
        taskProgress,
      ]),
    );

    return orderedTasks.find((task) => {
      const taskProgress = currentProgressByTaskId.get(task.id);

      return (
        taskProgress?.access_status === "AVAILABLE" &&
        taskProgress?.progress_status !== "COMPLETED"
      );
    });
  };

  const handleTaskCompleted = async () => {
    try {
      const updatedProgress = await reloadProgress();

      const nextTask = findNextAvailableTask(updatedProgress);

      if (nextTask) {
        setSelectedTask(nextTask);
        return;
      }

      const orderedTasks = getOrderedTasks();
      const lastTask = orderedTasks[orderedTasks.length - 1];

      if (lastTask) {
        setSelectedTask(lastTask);
      }
    } catch (err) {
      console.error("Failed to refresh progress after task completion:", err);
    }
  };

  useEffect(() => {
    if (!course || !progress || selectedTask) {
      return;
    }

    const nextTask = findNextAvailableTask(progress);

    if (nextTask) {
      setSelectedTask(nextTask);
      return;
    }

    const orderedTasks = getOrderedTasks();
    const lastTask = orderedTasks[orderedTasks.length - 1];

    if (lastTask) {
      setSelectedTask(lastTask);
    }
  }, [course, progress, selectedTask]);

  if (loading || progressLoading) {
    return <div className="p-8 text-zinc-300">Loading course...</div>;
  }

  if (error) {
    return <div className="p-8 text-red-400">{error}</div>;
  }

  if (progressError) {
    return <div className="p-8 text-red-400">{progressError}</div>;
  }

  if (!course) {
    return <div className="p-8 text-zinc-400">Course was not found.</div>;
  }

  return (
    <div className="flex h-full w-full flex-col px-6 py-6">
      <CourseHeader course={course} />

      {progress && (
        <div className="mt-5 rounded-xl border border-zinc-800 bg-zinc-950/60 p-4">
          <div className="flex items-center justify-between gap-4">
            <div>
              <p className="text-sm font-medium text-white">Course Progress</p>

              <p className="mt-1 text-xs text-zinc-500">
                {progress.completed_tasks} of {progress.total_tasks} tasks
                completed
              </p>
            </div>

            <div className="text-right">
              <p className="text-lg font-bold text-emerald-400">
                {progress.progress_percentage}%
              </p>

              <p className="text-xs text-zinc-500">
                {progress.earned_points} / {progress.total_points} points
              </p>
            </div>
          </div>

          <div className="mt-4 h-2 overflow-hidden rounded-full bg-zinc-800">
            <div
              className="h-full rounded-full bg-emerald-500 transition-all duration-500"
              style={{
                width: `${Math.min(
                  Math.max(progress.progress_percentage, 0),
                  100,
                )}%`,
              }}
            />
          </div>
        </div>
      )}

      <div className="mt-8 flex min-h-0 flex-1 gap-8">
        <SectionSidebar
          sections={course.sections || []}
          selectedTask={selectedTask}
          progressByTaskId={progressByTaskId}
          onSelectTask={setSelectedTask}
        />

        <main className="min-h-0 flex-1">
          <CourseWorkspace
            task={selectedTask}
            onTaskCompleted={handleTaskCompleted}
          />
        </main>
      </div>
    </div>
  );
}
