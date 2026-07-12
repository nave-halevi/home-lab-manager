import { useEffect, useState } from "react";
import { getCourse } from "../services/academyService";

export default function useCourse(id) {
  const [course, setCourse] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    if (!id) return;

    const loadCourse = async () => {
      try {
        const data = await getCourse(id);
        setCourse(data);
      } catch (err) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };

    loadCourse();
  }, [id]);

  return {
    course,
    loading,
    error,
  };
}
