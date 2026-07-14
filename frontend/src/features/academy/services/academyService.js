const BASE_URL = "http://localhost:3000/api/academy";

const getAuthHeaders = () => {
  const token = localStorage.getItem("token");

  return {
    "Content-Type": "application/json",
    Authorization: token ? `Bearer ${token}` : "",
  };
};

export const getCourses = async () => {
  const response = await fetch(`${BASE_URL}/courses`, {
    method: "GET",
    headers: getAuthHeaders(),
  });

  const data = await response.json();

  if (!response.ok) {
    throw new Error(data.message || "Failed to fetch courses");
  }

  return data;
};

export const getCourse = async (id) => {
  const response = await fetch(`${BASE_URL}/courses/${id}/full`, {
    method: "GET",
    headers: getAuthHeaders(),
  });

  const data = await response.json();

  if (!response.ok) {
    throw new Error(data.message || "Failed to fetch course");
  }

  return data;
};

export const getCourseProgress = async (courseId) => {
  if (!courseId) {
    throw new Error("Course ID is required");
  }

  const token = localStorage.getItem("token");

  const response = await fetch(
    `http://localhost:3000/api/task-progress/courses/${courseId}`,
    {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        Authorization: token ? `Bearer ${token}` : "",
      },
    },
  );

  const data = await response.json().catch(() => null);

  if (!response.ok) {
    throw new Error(data?.message || "Failed to fetch course progress");
  }

  return data;
};

export const completeContentTask = async (taskId) => {
  if (!taskId) {
    throw new Error("Task ID is required");
  }

  const token = localStorage.getItem("token");

  const response = await fetch(
    `http://localhost:3000/api/task-progress/tasks/${taskId}/complete`,
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: token ? `Bearer ${token}` : "",
      },
    },
  );

  const data = await response.json().catch(() => null);

  if (!response.ok) {
    throw new Error(data?.message || "Failed to complete task");
  }

  return data;
};

export const startTask = async (taskId) => {
  if (!taskId) {
    throw new Error("Task ID is required");
  }

  const token = localStorage.getItem("token");

  const response = await fetch(
    `http://localhost:3000/api/task-progress/tasks/${taskId}/start`,
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: token ? `Bearer ${token}` : "",
      },
    },
  );

  const data = await response.json().catch(() => null);

  if (!response.ok) {
    throw new Error(data?.message || "Failed to start task");
  }

  return data;
};
