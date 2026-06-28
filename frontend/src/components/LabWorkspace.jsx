import React from "react";
import TerminalWrapper from "./TerminalWrapper";
import { useCTF } from "../hooks/useCTF";
import { styles } from "./LabWorkspace.styles"; // מייבאים את העיצוב מהקובץ הנפרד

const LabWorkspace = ({ activeLab, onDeleteLab }) => {
  // החיבור למוח של המערכת (ה-Hook)
  const { flagInput, setFlagInput, feedback, isSubmitting, handleSubmit } =
    useCTF(activeLab.envId);

  return (
    <div style={styles.container}>
      {/* Left Side - Study & Task Panel */}
      <div style={styles.taskPanel}>
        <div style={styles.header}>
          <div
            style={{
              display: "flex",
              justifyContent: "space-between",
              alignItems: "flex-start",
            }}
          >
            <div>
              <h2 style={{ marginTop: 0, marginBottom: "5px" }}>
                Lab 1: Linux Fundamentals
              </h2>
              <span style={styles.badge}>Difficulty: Easy</span>
            </div>
            <button style={styles.terminateBtn} onClick={onDeleteLab}>
              🛑 Terminate Lab
            </button>
          </div>
        </div>

        <p style={styles.description}>
          Welcome to the training zone. In this lab, you will learn how to
          navigate the Linux file system and discover hidden information.
        </p>

        <div style={styles.taskBox}>
          <h3 style={{ color: "#4CAF50", marginTop: 0 }}>
            🚩 Current Objective: Capture The Flag
          </h3>
          <p>
            Navigate to the <code>/tmp</code> directory in the system. Inside,
            there is a hidden text file named <code>flag.txt</code>. Output its
            contents using the <code>cat</code> command, and submit the flag you
            found below to complete the lab.
          </p>

          <input
            type="text"
            placeholder="Enter the flag (e.g., CTF{...})"
            style={styles.input}
            value={flagInput}
            onChange={(e) => setFlagInput(e.target.value)}
          />

          <button
            style={styles.button}
            onClick={(e) => {
              e.preventDefault(); 
              handleSubmit();
            }}
          >
            {isSubmitting ? "Checking..." : "Submit Flag"}
          </button>

          {feedback && (
            <div
              style={{
                marginTop: "15px",
                padding: "10px",
                borderRadius: "4px",
                textAlign: "center",
                fontWeight: "bold",
                backgroundColor: feedback.includes("✅")
                  ? "rgba(35, 134, 54, 0.2)"
                  : "rgba(248, 81, 73, 0.2)",
                color: feedback.includes("✅") ? "#4CAF50" : "#f85149",
                border: `1px solid ${feedback.includes("✅") ? "#4CAF50" : "#f85149"}`,
              }}
            >
              {feedback}
            </div>
          )}
        </div>
      </div>

      {/* Right Side - Live Terminal */}
      <div style={styles.terminalPanel}>
        <TerminalWrapper activeLab={activeLab} />
      </div>
    </div>
  );
};

export default LabWorkspace;