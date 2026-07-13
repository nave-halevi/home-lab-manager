import { useCallback, useState } from "react";

import { submitLabFlag } from "../../labs/services/labService";

export const useCTF = (environmentId, taskId) => {
  const [flagInput, setFlagInput] = useState("");
  const [feedback, setFeedback] = useState(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isSolved, setIsSolved] = useState(false);

  const clearFeedback = useCallback(() => {
    setFeedback(null);
  }, []);

  const handleSubmit = useCallback(async () => {
    const normalizedFlag = flagInput.trim();

    if (!environmentId) {
      setFeedback("⚠️ Start the lab machine before submitting a flag.");
      return false;
    }

    if (!taskId) {
      setFeedback("❌ This lab is not connected to a task.");
      return false;
    }

    if (!normalizedFlag) {
      setFeedback("⚠️ Please enter a flag first.");
      return false;
    }

    setIsSubmitting(true);
    setFeedback(null);

    try {
      const data = await submitLabFlag(environmentId, taskId, normalizedFlag);

      const message = data?.message || "The server did not return a message.";

      setFeedback(message);

      const solved =
        message.includes("✅") ||
        message.includes("already submitted") ||
        message.toLowerCase().includes("correct");

      if (solved) {
        setIsSolved(true);
        setFlagInput("");
      }

      return solved;
    } catch (error) {
      setFeedback(`❌ ${error.message || "Failed to submit flag."}`);
      return false;
    } finally {
      setIsSubmitting(false);
    }
  }, [environmentId, taskId, flagInput]);

  return {
    flagInput,
    setFlagInput,
    feedback,
    isSubmitting,
    isSolved,
    handleSubmit,
    clearFeedback,
  };
};
