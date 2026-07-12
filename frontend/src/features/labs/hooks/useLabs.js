import { useCallback, useState } from "react";

import { useAuth } from "../../../context/AuthContext";

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
  const { user } = useAuth();

  const [activeLab, setActiveLab] = useState(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);

  const restoreActiveLab = useCallback(
    async (scenarioId) => {
      if (!user?.id || !scenarioId) {
        setActiveLab(null);
        return null;
      }

      setIsLoading(true);
      setError(null);

      try {
        const data = await getActiveLab(user.id, scenarioId);

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
    },
    [user?.id],
  );

  const handleCreateLab = useCallback(
    async (scenarioId) => {
      if (!user?.id) {
        setError("Authenticated user was not found.");
        return null;
      }

      if (!scenarioId) {
        setError("This task is not connected to a scenario.");
        return null;
      }

      setIsLoading(true);
      setError(null);

      try {
        const data = await createLab(scenarioId, user.id);

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

        /*
          The UI may have forgotten an already-running lab.
          Try restoring it before leaving the user with an error.
        */
        try {
          const existingLab = await getActiveLab(user.id, scenarioId);

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
    },
    [user?.id],
  );

  const handleDeleteLab = useCallback(async () => {
    if (!activeLab?.envId) {
      return false;
    }

    if (!user?.id) {
      setError("Authenticated user was not found.");
      return false;
    }

    setIsLoading(true);
    setError(null);

    try {
      await deleteLab(user.id, activeLab.envId);

      setActiveLab(null);

      return true;
    } catch (requestError) {
      setError(requestError.message);

      return false;
    } finally {
      setIsLoading(false);
    }
  }, [activeLab?.envId, user?.id]);

  const clearLabError = useCallback(() => {
    setError(null);
  }, []);

  return {
    user,
    activeLab,
    isLoading,
    error,
    restoreActiveLab,
    handleCreateLab,
    handleDeleteLab,
    clearLabError,
  };
};
