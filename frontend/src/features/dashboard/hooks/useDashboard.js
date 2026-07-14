import { useCallback, useEffect, useState } from "react";

import { getDashboard } from "../services/dashboardService";

export default function useDashboard() {
  const [dashboard, setDashboard] = useState(null);

  const [loading, setLoading] = useState(true);

  const [error, setError] = useState(null);

  const loadDashboard = useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      const data = await getDashboard();

      setDashboard(data);

      return data;
    } catch (requestError) {
      setError(requestError.message || "Failed to load dashboard.");

      throw requestError;
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadDashboard().catch(() => {
      // The error is already stored in state.
    });
  }, [loadDashboard]);

  return {
    dashboard,
    loading,
    error,
    reloadDashboard: loadDashboard,
  };
}
