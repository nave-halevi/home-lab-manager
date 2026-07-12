import { useNavigate } from "react-router-dom";

import Card from "../../../shared/ui/Card";
import Button from "../../../shared/ui/Button";

export default function CourseCard({ course }) {
  const navigate = useNavigate();

  return (
    <Card>
      <h2 className="text-2xl font-bold mb-3">{course.title}</h2>

      <p className="text-zinc-400 mb-6">{course.description}</p>

      <div className="flex gap-4 mb-6">
        <span className="text-sm text-zinc-300">{course.difficulty}</span>

        <span className="text-sm text-red-400">
          {course.is_published ? "Published" : "Draft"}
        </span>
      </div>

      <Button onClick={() => navigate(`/academy/${course.id}`)}>
        Open Course
      </Button>
    </Card>
  );
}
