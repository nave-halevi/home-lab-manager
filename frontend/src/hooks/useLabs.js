import { useState } from "react";
import { creatLab } from "../services/labService";

export const useLabs = () => {
  const [activeLab, setActiveLab] = useState(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);

  const handleCreateLab = async () => {
    setIsLoading(true);
    setError(null);

    try {
      const data = await creatLab();
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
  return {
    activeLab,
    isLoading,
    error,
    handleCreateLab,
  };
};
