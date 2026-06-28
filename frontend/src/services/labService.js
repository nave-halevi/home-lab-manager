// מגדירים את הכתובת פעם אחת - קל לשנות כשעוברים לשרת אמיתי
const BASE_URL = "http://localhost:3000/api/lab";

export const createLab = async () => {
  try {
    const response = await fetch(`${BASE_URL}/create`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        // כרגע נשאיר קשיח לבדיקות, בעתיד נעביר לפה פרמטרים
        user_id: "91baef38-0364-44c9-9a9a-748b31d1945e",
        scenario_id: "98b7d92f-7fbb-446c-90e5-df05aea4d27f",
      }),
    });

    if (!response.ok) {
      throw new Error("Error creating the lab on the server.");
    }

    return await response.json();
  } catch (error) {
    console.error("LabService Error:", error);
    throw error;
  }
};

export const submitLabFlag = async (envId, flagValue) => {
  const response = await fetch(`${BASE_URL}/submit`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
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
