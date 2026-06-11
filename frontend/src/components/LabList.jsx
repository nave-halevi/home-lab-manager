import React from "react";

const LabList = ({ activeLab }) => {
  if (!activeLab) {
    return null;
  }
  return (
    <div
      style={{
        marginTop: "30px",
        padding: "20px",
        border: "1px solid #ddd",
        borderRadius: "8px",
      }}
    >
      <h2 style={{ color: "green" }}>✅ Lab is up and running!</h2>
      <p>
        Internal SSH Port: <strong>{activeLab.port}</strong>
      </p>
      <p>
        Environment ID:{" "}
        <span style={{ color: "gray", fontSize: "12px" }}>
          {activeLab.envId}
        </span>
      </p>

      <div
        style={{
          marginTop: "20px",
          height: "300px",
          backgroundColor: "#1e1e1e",
          color: "#fff",
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          fontFamily: "monospace",
        }}
      >
        [Terminal will be injected here in the next step]
      </div>
    </div>
  );
};
export default LabList;
