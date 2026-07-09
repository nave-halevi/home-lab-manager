const BASE_URL = "http://localhost:3000/api/lab";

const getAuthHeaders = () => {
  const token = localStorage.getItem("token");

  return {
    "Content-Type": "application/json",
    Authorization: token ? `Bearer ${token}` : "",
  };
};

export const createLab = async (scenarioId = "98b7d92f-7fbb-446c-90e5-df05aea4d27f", userId = null) => {
  try {
    const headers = getAuthHeaders();
    console.log("🔐 Lab create auth headers:", headers);
    console.log("🪪 Stored token exists:", !!localStorage.getItem("token"));

    const response = await fetch(`${BASE_URL}/create`, {
      method: "POST",
      headers,
      body: JSON.stringify({
        user_id: userId,
        scenario_id: scenarioId,
      }),
    });

    const data = await response.json().catch(() => ({}));

    if (!response.ok) {
      throw new Error(data.message || "Error creating the lab on the server.");
    }

    return data;
  } catch (error) {
    console.error("LabService Error:", error);
    throw error;
  }
};

export const submitLabFlag = async (envId, flagValue) => {
  const response = await fetch(`${BASE_URL}/submit`, {
    method: "POST",
    headers: getAuthHeaders(),
    body: JSON.stringify({
      env_id: envId,
      flag: flagValue,
    }),
  });

  if (!response.ok) {
    throw new Error("Failed to connect to server");
  }

  return await response.json();
};
