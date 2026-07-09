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
