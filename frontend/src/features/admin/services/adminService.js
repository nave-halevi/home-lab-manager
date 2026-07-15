const API_ORIGIN = import.meta.env.VITE_API_URL || "http://localhost:3000";
const ADMIN_URL = `${API_ORIGIN}/api/admin`;
const ACADEMY_URL = `${API_ORIGIN}/api/academy/admin`;

const headers = () => ({
  "Content-Type": "application/json",
  Authorization: `Bearer ${localStorage.getItem("token") || ""}`,
});

async function request(url, options = {}) {
  const response = await fetch(url, { ...options, headers: headers() });
  if (response.status === 204 || response.status === 200 && options.method === "DELETE") return null;
  const data = await response.json().catch(() => null);
  if (!response.ok) throw new Error(data?.message || data?.error || `Request failed with status ${response.status}`);
  return data;
}

export const getAdminDashboard = () => request(`${ADMIN_URL}/dashboard`);
export const getAdminUsers = () => request(`${ADMIN_URL}/users`);
export const getAdminLabs = () => request(`${ADMIN_URL}/labs`);
export const getAdminFlags = () => request(`${ADMIN_URL}/flags`);
export const getAdminScenarios = () => request(`${ADMIN_URL}/scenarios`);
export const createScenario = (body) => request(`${ADMIN_URL}/scenarios`, { method: "POST", body: JSON.stringify(body) });
export const updateScenario = (id, body) => request(`${ADMIN_URL}/scenarios/${id}`, { method: "PUT", body: JSON.stringify(body) });
export const deleteScenario = (id) => request(`${ADMIN_URL}/scenarios/${id}`, { method: "DELETE" });

export const getAdminCourses = () => request(`${ACADEMY_URL}/courses`);
export const createCourse = (body) => request(`${ACADEMY_URL}/courses`, { method: "POST", body: JSON.stringify(body) });
export const updateCourse = (id, body) => request(`${ACADEMY_URL}/courses/${id}`, { method: "PUT", body: JSON.stringify(body) });
export const deleteCourse = (id) => request(`${ACADEMY_URL}/courses/${id}`, { method: "DELETE" });
export const getSectionsByCourse = (id) => request(`${ACADEMY_URL}/courses/${id}/sections`);
export const createSection = (body) => request(`${ACADEMY_URL}/sections`, { method: "POST", body: JSON.stringify(body) });
export const updateSection = (id, body) => request(`${ACADEMY_URL}/sections/${id}`, { method: "PUT", body: JSON.stringify(body) });
export const deleteSection = (id) => request(`${ACADEMY_URL}/sections/${id}`, { method: "DELETE" });
export const getTasksBySection = (id) => request(`${ACADEMY_URL}/sections/${id}/tasks`);
export const createTask = (body) => request(`${ACADEMY_URL}/tasks`, { method: "POST", body: JSON.stringify(body) });
export const updateTask = (id, body) => request(`${ACADEMY_URL}/tasks/${id}`, { method: "PUT", body: JSON.stringify(body) });
export const deleteTask = (id) => request(`${ACADEMY_URL}/tasks/${id}`, { method: "DELETE" });
