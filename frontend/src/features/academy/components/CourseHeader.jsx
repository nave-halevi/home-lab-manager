export default function CourseHeader({ course }) {
  return (
    <div
      className="
      text-center
      border-b
      border-zinc-800
      pb-8
      mb-8
      "
    >
      <h1
        className="
        text-4xl
        font-bold
        text-white
        "
      >
        {course.title}
      </h1>

      <p
        className="
        mt-3
        text-zinc-400
        max-w-2xl
        mx-auto
        "
      >
        {course.description}
      </p>

      <div
        className="
        mt-5
        flex
        justify-center
        gap-3
      "
      >
        <span>{course.difficulty}</span>

        <span>{course.is_published ? "Published" : "Draft"}</span>
      </div>
    </div>
  );
}
