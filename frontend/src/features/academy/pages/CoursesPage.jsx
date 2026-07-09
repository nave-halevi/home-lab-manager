import CourseCard from "../components/CourseCard";
import useCourses from "../hooks/useCourses";

export default function CoursesPage() {
  const { courses, loading, error } = useCourses();

  if (loading) {
    return <div className="p-8 text-zinc-300">Loading courses...</div>;
  }

  if (error) {
    return <div className="p-8 text-red-400">{error}</div>;
  }

  return (
    <div className="mx-auto max-w-7xl p-8">
      <div className="mb-10">
        <h1 className="text-4xl font-bold text-white">Academy</h1>

        <p className="mt-2 text-zinc-400">
          Explore cybersecurity courses and improve your skills.
        </p>
      </div>

      <div className="grid gap-6 md:grid-cols-2">
        {courses.map((course) => (
          <CourseCard key={course.id} course={course} />
        ))}
      </div>
    </div>
  );
}
