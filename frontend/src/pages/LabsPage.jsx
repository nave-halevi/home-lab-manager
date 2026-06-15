import React from "react";
import { useLabs } from "../hooks/useLabs";
import CreateLabButton from "../components/CreateLabButton";
import LabWorkspace from "../components/LabWorkspace";

const LabsPage = () => {
  const { activeLab, isLoading, error, handleCreateLab, handleDeleteLab } = useLabs();
 
 
 
  console.log("Current activeLab state:", activeLab);


  if (activeLab) {
    console.log("LabWorkspace rendered with props:", {
      activeLab,
      onDeleteLab: handleDeleteLab,
    });
    return <LabWorkspace activeLab={activeLab} onDeleteLab={handleDeleteLab} />;
  }

  // Otherwise, render the standard creation dashboard
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

      <CreateLabButton isLoading={isLoading} onCreate={handleCreateLab} />

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
