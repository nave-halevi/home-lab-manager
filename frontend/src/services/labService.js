export const creatLab = async () => {
  try {
    const response = await fetch("/api/lab/create", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
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
