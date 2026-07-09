import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

import useCourse from "../hooks/useCourse";

import CourseHeader from "../components/CourseHeader";
import SectionSidebar from "../components/SectionSidebar";

import CourseWorkspace from "../components/workspace/CourseWorkspace";
export default function CoursePage() {
  const { courseId } = useParams();

  const { course, loading, error } = useCourse(courseId);

  const [selectedTask, setSelectedTask] = useState(null);

  useEffect(() => {
    if (!course) return;

    if (selectedTask) return;

    const firstTask = course.sections?.[0]?.tasks?.[0];

    if (firstTask) {
      setSelectedTask(firstTask);
    }
  }, [course, selectedTask]);

  if (loading) {
    return <div className="p-8 text-zinc-300">Loading course...</div>;
  }

  if (error) {
    return <div className="p-8 text-red-400">{error}</div>;
  }

  return (
    <div className="w-full h-full px-6 py-6 flex flex-col">
      {" "}
      <CourseHeader course={course} />
      <div className="flex gap-8 mt-8 flex-1 min-h-0">
        {" "}
        <SectionSidebar
          sections={course.sections}
          selectedTask={selectedTask}
          onSelectTask={setSelectedTask}
        />
        <main className="flex-1 min-h-0">
          <CourseWorkspace task={selectedTask} />
        </main>
      </div>
    </div>
  );
}
