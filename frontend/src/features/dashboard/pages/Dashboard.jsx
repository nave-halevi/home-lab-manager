import { useNavigate } from "react-router-dom";

import useDashboard from "../hooks/useDashboard";

export default function Dashboard() {
  const navigate = useNavigate();

  const { dashboard, loading, error } = useDashboard();

  if (loading) {
    return <div className="p-10 text-zinc-300">Loading dashboard... </div>;
  }

  if (error) {
    return <div className="p-10 text-red-400">{error} </div>;
  }

  if (!dashboard) {
    return (
      <div className="p-10 text-zinc-400">Dashboard data was not found. </div>
    );
  }

  const continueLearning = dashboard.continue_learning || [];

  const completedCourses = dashboard.completed_courses || [];

  const availableCourses = dashboard.available_courses || [];

  return (
    <main className="min-h-screen bg-zinc-950 px-6 py-10 text-white">
      {" "}
      <section className="mb-10">
        {" "}
        <h1 className="text-3xl font-bold">
          Welcome back,{" "}
          <span className="text-red-500">{dashboard.user.user_name} </span>{" "}
        </h1>
        <p className="mt-2 text-zinc-400">
          Continue your cyber security learning path.
        </p>
      </section>
      <section className="mb-10">
        <h2 className="mb-4 text-xl font-semibold">Your Progress</h2>

        <div className="grid gap-6 md:grid-cols-2 xl:grid-cols-4">
          <div className="rounded-xl border border-zinc-800 bg-zinc-900 p-5">
            <p className="text-sm text-zinc-400">Total Score</p>

            <p className="mt-2 text-3xl font-bold text-emerald-400">
              {dashboard.user.total_score}
            </p>
          </div>

          <div className="rounded-xl border border-zinc-800 bg-zinc-900 p-5">
            <p className="text-sm text-zinc-400">Active Courses</p>

            <p className="mt-2 text-3xl font-bold">
              {dashboard.statistics.active_courses}
            </p>
          </div>

          <div className="rounded-xl border border-zinc-800 bg-zinc-900 p-5">
            <p className="text-sm text-zinc-400">Completed Courses</p>

            <p className="mt-2 text-3xl font-bold">
              {dashboard.statistics.completed_courses}
            </p>
          </div>

          <div className="rounded-xl border border-zinc-800 bg-zinc-900 p-5">
            <p className="text-sm text-zinc-400">Completed Tasks</p>

            <p className="mt-2 text-3xl font-bold">
              {dashboard.statistics.completed_tasks}
            </p>
          </div>
        </div>
      </section>
      <section className="mb-10">
        <div className="mb-4 flex items-center justify-between gap-4">
          <h2 className="text-xl font-semibold">Continue Learning</h2>

          {continueLearning.length > 0 && (
            <span className="text-sm text-zinc-500">
              {continueLearning.length} active{" "}
              {continueLearning.length === 1 ? "course" : "courses"}
            </span>
          )}
        </div>

        {continueLearning.length === 0 ? (
          <div className="rounded-xl border border-zinc-800 bg-zinc-900 p-6">
            <h3 className="text-lg font-semibold">Start your first course</h3>

            <p className="mt-2 text-sm text-zinc-400">
              Choose a course below and begin your learning journey.
            </p>
          </div>
        ) : (
          <div className="grid gap-6 md:grid-cols-2 xl:grid-cols-3">
            {continueLearning.map((course) => (
              <article
                key={course.course_id}
                className="flex flex-col rounded-xl border border-zinc-800 bg-zinc-900 p-5 transition hover:border-zinc-700"
              >
                <div className="flex-1">
                  <div className="flex items-start justify-between gap-4">
                    <h3 className="text-lg font-semibold">
                      {course.course_title}
                    </h3>

                    {course.difficulty && (
                      <span className="shrink-0 rounded-full bg-zinc-800 px-2.5 py-1 text-xs text-zinc-300">
                        {course.difficulty}
                      </span>
                    )}
                  </div>

                  {course.course_description && (
                    <p className="mt-2 line-clamp-2 text-sm text-zinc-400">
                      {course.course_description}
                    </p>
                  )}

                  {course.current_task && (
                    <div className="mt-4 rounded-lg border border-zinc-800 bg-zinc-950/60 p-3">
                      <p className="text-xs uppercase tracking-wide text-zinc-500">
                        Current task
                      </p>

                      <p className="mt-1 text-sm font-medium text-zinc-200">
                        {course.current_task.task_title}
                      </p>

                      <p className="mt-1 text-xs text-zinc-500">
                        {course.current_task.task_type}
                      </p>
                    </div>
                  )}

                  <div className="mt-5 h-2 overflow-hidden rounded-full bg-zinc-800">
                    <div
                      className="h-full rounded-full bg-red-600 transition-all duration-500"
                      style={{
                        width: `${Math.min(
                          Math.max(course.progress_percentage, 0),
                          100,
                        )}%`,
                      }}
                    />
                  </div>

                  <div className="mt-3 flex items-center justify-between text-sm text-zinc-400">
                    <span>
                      {course.completed_tasks}/{course.total_tasks} tasks
                    </span>

                    <span>{course.progress_percentage}%</span>
                  </div>

                  <div className="mt-2 text-xs text-zinc-500">
                    {course.earned_points} / {course.total_points} points
                  </div>
                </div>

                <button
                  type="button"
                  onClick={() => navigate(`/academy/${course.course_id}`)}
                  className="mt-5 w-full rounded-lg bg-red-600 px-4 py-2.5 text-sm font-semibold text-white transition hover:bg-red-500"
                >
                  Continue Course →
                </button>
              </article>
            ))}
          </div>
        )}
      </section>
      {completedCourses.length > 0 && (
        <section className="mb-10">
          <div className="mb-4 flex items-center justify-between gap-4">
            <h2 className="text-xl font-semibold">Completed Courses</h2>

            <span className="text-sm text-emerald-400">
              {completedCourses.length} completed
            </span>
          </div>

          <div className="grid gap-6 md:grid-cols-2 xl:grid-cols-3">
            {completedCourses.map((course) => (
              <article
                key={course.course_id}
                className="flex flex-col rounded-xl border border-emerald-900/70 bg-emerald-950/20 p-5"
              >
                <div className="flex-1">
                  <div className="flex items-start justify-between gap-4">
                    <div>
                      <p className="text-xs font-semibold uppercase tracking-wide text-emerald-400">
                        ✓ Completed
                      </p>

                      <h3 className="mt-2 text-lg font-semibold">
                        {course.course_title}
                      </h3>
                    </div>

                    {course.difficulty && (
                      <span className="shrink-0 rounded-full bg-zinc-800 px-2.5 py-1 text-xs text-zinc-300">
                        {course.difficulty}
                      </span>
                    )}
                  </div>

                  {course.course_description && (
                    <p className="mt-3 line-clamp-2 text-sm text-zinc-400">
                      {course.course_description}
                    </p>
                  )}

                  <div className="mt-5 h-2 overflow-hidden rounded-full bg-zinc-800">
                    <div className="h-full w-full rounded-full bg-emerald-500" />
                  </div>

                  <div className="mt-3 flex items-center justify-between text-sm text-zinc-400">
                    <span>
                      {course.completed_tasks}/{course.total_tasks} tasks
                    </span>

                    <span className="text-emerald-400">
                      {course.progress_percentage}%
                    </span>
                  </div>

                  <div className="mt-2 text-xs text-zinc-500">
                    {course.earned_points} / {course.total_points} points earned
                  </div>
                </div>

                <button
                  type="button"
                  onClick={() => navigate(`/academy/${course.course_id}`)}
                  className="mt-5 w-full rounded-lg border border-emerald-800 px-4 py-2.5 text-sm font-semibold text-emerald-300 transition hover:border-emerald-600 hover:text-emerald-200"
                >
                  Review Course →
                </button>
              </article>
            ))}
          </div>
        </section>
      )}
      <section>
        <div className="mb-4 flex items-center justify-between gap-4">
          <h2 className="text-xl font-semibold">Explore Courses</h2>

          {availableCourses.length > 0 && (
            <span className="text-sm text-zinc-500">
              {availableCourses.length} available
            </span>
          )}
        </div>

        {availableCourses.length === 0 ? (
          <div className="rounded-xl border border-zinc-800 bg-zinc-900 p-6 text-sm text-zinc-400">
            No additional courses are currently available.
          </div>
        ) : (
          <div className="grid gap-6 md:grid-cols-2 xl:grid-cols-3">
            {availableCourses.map((course) => (
              <article
                key={course.course_id}
                className="flex flex-col rounded-xl border border-zinc-800 bg-zinc-900 p-5 transition hover:border-zinc-700"
              >
                <div className="flex-1">
                  <div className="flex items-start justify-between gap-4">
                    <h3 className="text-lg font-semibold">
                      {course.course_title}
                    </h3>

                    {course.difficulty && (
                      <span className="shrink-0 rounded-full bg-zinc-800 px-2.5 py-1 text-xs text-zinc-300">
                        {course.difficulty}
                      </span>
                    )}
                  </div>

                  <p className="mt-2 text-sm text-zinc-400">
                    {course.course_description ||
                      "Start learning and build practical skills."}
                  </p>

                  <div className="mt-5 flex items-center justify-between text-sm text-zinc-500">
                    <span>{course.total_tasks} tasks</span>

                    <span>{course.total_points} points</span>
                  </div>
                </div>

                <button
                  type="button"
                  onClick={() => navigate(`/academy/${course.course_id}`)}
                  className="mt-5 w-full rounded-lg border border-zinc-700 px-4 py-2.5 text-sm font-semibold text-zinc-200 transition hover:border-red-500 hover:text-white"
                >
                  Start Course →
                </button>
              </article>
            ))}
          </div>
        )}
      </section>
    </main>
  );
}
