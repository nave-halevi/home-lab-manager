import { useState } from "react";
import { useAuth } from "../../../context/AuthContext";
import { createLab } from "../services/labService";

export const useLabs = () => {
  const { user } = useAuth();
  const [activeLab, setActiveLab] = useState(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);

  const handleCreateLab = async (scenarioId) => {
    setIsLoading(true);
    setError(null);

    try {
      const data = await createLab(scenarioId, user?.id);
      console.log("📦 Data received from Rust:", data);

      setActiveLab({
        envId: data.env_id,
        port: data.ssh_port,
      });
    } catch (err) {
      setError(err.message);
    } finally {
      setIsLoading(false);
    }
  };
  
  const handleDeleteLab = async () => {
    if (!activeLab) return;

    try {
      // אנחנו משתמשים ב-POST ושולחים את המזהה ב-Body, בדיוק כמו שהראסט מצפה
      const response = await fetch("http://localhost:3000/api/lab/delete", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ env_id: activeLab.envId }), // שים לב שהשתמשנו ב-envId
      });

      if (!response.ok) {
        throw new Error("Failed to delete lab on the server");
      }

      // מנקים את המעבדה מהזיכרון של React כדי שהאתר יחזור לדאשבורד
      setActiveLab(null);
      console.log("[SUCCESS] Lab deleted successfully!");
    } catch (err) {
      console.error("[ERROR] Failed to terminate lab:", err);
      setError(
        "Failed to terminate the lab. You might need to delete it manually.",
      );
    }
  };

  return {
    activeLab,
    isLoading,
    error,
    handleCreateLab,
    handleDeleteLab,
  };
};
