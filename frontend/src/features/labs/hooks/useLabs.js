import { useCallback, useState } from "react";

import { createLab, deleteLab, getActiveLab } from "../services/labService";

function mapActiveLabResponse(data) {
  if (!data) {
    return null;
  }

  return {
    envId: data.environment_id,
    instanceId: data.instance_id || null,
    scenarioId: data.scenario_id,
    sshPort: data.ssh_port || null,
    status: data.environment_status,
    instanceStatus: data.instance_status || null,
    vmName: data.vm_name || null,
  };
}

export const useLabs = () => {
  const [activeLab, setActiveLab] = useState(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);

  const restoreActiveLab = useCallback(async (scenarioId) => {
    if (!scenarioId) {
      setActiveLab(null);
      return null;
    }

    setIsLoading(true);
    setError(null);

    try {
      const data = await getActiveLab(scenarioId);

      const restoredLab = mapActiveLabResponse(data);

      setActiveLab(restoredLab);

      return restoredLab;
    } catch (requestError) {
      setActiveLab(null);
      setError(requestError.message);

      return null;
    } finally {
      setIsLoading(false);
    }
  }, []);

  const handleCreateLab = useCallback(async (scenarioId) => {
    if (!scenarioId) {
      setError("This task is not connected to a scenario.");
      return null;
    }

    setIsLoading(true);
    setError(null);

    try {
      const data = await createLab(scenarioId);

      const createdLab = {
        envId: data.env_id,
        instanceId: data.instance_id || null,
        scenarioId,
        sshPort: data.ssh_port || null,
        status: "Running",
        instanceStatus: "Running",
        vmName: data.vm_name || null,
      };

      setActiveLab(createdLab);

      return createdLab;
    } catch (requestError) {
      setError(requestError.message);

      try {
        const existingLab = await getActiveLab(scenarioId);

        const restoredLab = mapActiveLabResponse(existingLab);

        if (restoredLab) {
          setActiveLab(restoredLab);
          setError(null);

          return restoredLab;
        }
      } catch {
        // Preserve the original create error.
      }

      return null;
    } finally {
      setIsLoading(false);
    }
  }, []);

  const handleDeleteLab = useCallback(async () => {
    if (!activeLab?.envId) {
      return false;
    }

    setIsLoading(true);
    setError(null);

    try {
      await deleteLab(activeLab.envId);

      setActiveLab(null);

      return true;
    } catch (requestError) {
      setError(requestError.message);

      return false;
    } finally {
      setIsLoading(false);
    }
  }, [activeLab?.envId]);

  const clearLabError = useCallback(() => {
    setError(null);
  }, []);

  return {
    activeLab,
    isLoading,
    error,
    restoreActiveLab,
    handleCreateLab,
    handleDeleteLab,
    clearLabError,
  };
};
