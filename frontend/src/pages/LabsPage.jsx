import React from "react";
import { useLabs } from "../hooks/useLabs";
import CreateLabButton from "../components/CreateLabButton";
import LabList from "../components/LabList";

const LabsPage = () => {
  const { activeLab, isLoading, error, handleCreateLab } = useLabs();

  return (
    <div
      style={{
        maxWidth: "800px",
        margin: "50px auto",
        textAlign: "center",
        fontFamily: "sans-serif",
      }}
    >
      <h1>Cyber Security Home Lab 🚀</h1>

      {/* מעבירים את המצבים והפונקציות למטה כ-Props */}
      {!activeLab && (
        <CreateLabButton isLoading={isLoading} onCreate={handleCreateLab} />
      )}

      {/* מעבירים את אובייקט המעבדה למטה */}
      <LabList activeLab={activeLab} />

      {/* הצגת שגיאות במידה והשרת נכשל */}
      {error && (
        <div
          style={{
            color: "red",
            marginTop: "20px",
            padding: "10px",
            backgroundColor: "#ffe6e6",
          }}
        >
          [ERROR] {error}
        </div>
      )}
    </div>
  );
};
export default LabsPage;
