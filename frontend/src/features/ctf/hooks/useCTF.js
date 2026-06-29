import { useState } from 'react';
import { submitLabFlag } from '../../labs/services/labService';

export const useCTF = (envId) => {
  const [flagInput, setFlagInput] = useState('');
  const [feedback, setFeedback] = useState(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async () => {
    console.log("🔥 CLICK EVENT DETECTED!");
    console.log("🎯 הלחיצה נקלטה! ערך התיבה כרגע:", flagInput);

    // במקום פשוט לעצור בשקט, אנחנו נותנים פידבק למשתמש
    if (!flagInput || flagInput.trim() === '') {
      console.log("🛑 עצרתי לפני פנייה לשרת כי התיבה ריקה");
      setFeedback("⚠️ Please enter a flag first!");
      return;
    }

    setIsSubmitting(true);
    setFeedback(null);
    console.log("📡 פונה לשרת עם המזהה:", envId);

    try {
      const data = await submitLabFlag(envId, flagInput);
      console.log("✅ השרת ענה:", data);
      setFeedback(data.message);

      if (data.message.includes('✅')) {
        setFlagInput('');
      }
    } catch (error) {
      console.error("❌ שגיאת תקשורת מול השרת:", error);
      setFeedback('❌ Failed to connect to server.');
    } finally {
      setIsSubmitting(false);
    }
  };

  return {
    flagInput,
    setFlagInput,
    feedback,
    isSubmitting,
    handleSubmit
  };
};