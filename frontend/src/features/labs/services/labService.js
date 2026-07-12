const API_ORIGIN = import.meta.env.VITE_API_URL || "http://localhost:3000";

const BASE_URL = `${API_ORIGIN}/api/lab`;

const getAuthHeaders = () => {
  const token = localStorage.getItem("token");

  return {
    "Content-Type": "application/json",
    ...(token
      ? {
          Authorization: `Bearer ${token}`,
        }
      : {}),
  };
};

async function parseResponse(response) {
  const data = await response.json().catch(() => null);

  if (!response.ok) {
    const message =
      data?.message ||
      data?.error ||
      `Request failed with status ${response.status}`;

    throw new Error(message);
  }

  return data;
}

export async function createLab(scenarioId, userId) {
  if (!scenarioId) {
    throw new Error("Scenario ID is required to create a lab.");
  }

  if (!userId) {
    throw new Error("User ID is required to create a lab.");
  }

  const response = await fetch(`${BASE_URL}/create`, {
    method: "POST",
    headers: getAuthHeaders(),
    body: JSON.stringify({
      user_id: userId,
      scenario_id: scenarioId,
    }),
  });

  return parseResponse(response);
}

export async function deleteLab(userId, environmentId) {
  if (!userId) {
    throw new Error("User ID is required to delete a lab.");
  }

  if (!environmentId) {
    throw new Error("Environment ID is required to delete a lab.");
  }

  const response = await fetch(`${BASE_URL}/delete`, {
    method: "POST",
    headers: getAuthHeaders(),
    body: JSON.stringify({
      user_id: userId,
      env_id: environmentId,
    }),
  });

  return parseResponse(response);
}

export async function getActiveLab(userId, scenarioId) {
  if (!userId || !scenarioId) {
    return null;
  }

  const response = await fetch(`${BASE_URL}/active/${userId}/${scenarioId}`, {
    method: "GET",
    headers: getAuthHeaders(),
  });

  return parseResponse(response);
}

export async function getLabStatus(userId, environmentId) {
  if (!userId || !environmentId) {
    return null;
  }

  const response = await fetch(
    `${BASE_URL}/status/${userId}/${environmentId}`,
    {
      method: "GET",
      headers: getAuthHeaders(),
    },
  );

  return parseResponse(response);
}

export async function submitLabFlag(userId, environmentId, taskId, flagValue) {
  if (!userId) {
    throw new Error("User ID is required to submit a flag.");
  }

  if (!environmentId) {
    throw new Error("Start the lab machine before submitting a flag.");
  }

  if (!taskId) {
    throw new Error("Task ID is required to complete the lab.");
  }

  if (!flagValue?.trim()) {
    throw new Error("Flag value is required.");
  }

  const response = await fetch(`${BASE_URL}/submit`, {
    method: "POST",
    headers: getAuthHeaders(),
    body: JSON.stringify({
      user_id: userId,
      env_id: environmentId,
      task_id: taskId,
      flag: flagValue.trim(),
    }),
  });

  return parseResponse(response);
}

export function getTerminalUrl(environmentId) {
  if (!environmentId) {
    throw new Error("Environment ID is required for a terminal connection.");
  }

  const websocketOrigin = API_ORIGIN.replace(/^http/, "ws");

  return `${websocketOrigin}/api/lab/terminal/${environmentId}`;
}
