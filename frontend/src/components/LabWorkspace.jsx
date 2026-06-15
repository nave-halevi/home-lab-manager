import React from "react";
import TerminalWrapper from "./TerminalWrapper";

const LabWorkspace = ({ activeLab, onDeleteLab }) => {
  
  return (
    <div style={styles.container}>
      {/* Left Side - Study & Task Panel */}

      <div style={styles.taskPanel}>
        {/* הנה התיקון העיצובי: שמנו את שניהם בשורה אחת עם רווח ביניהם */}
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

        {/* ... המשך הקובץ שלך נשאר בדיוק אותו דבר ... */}
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
          />
          <button style={styles.button}>Submit Flag</button>
        </div>
      </div>

      {/* Right Side - Live Terminal */}
      <div style={styles.terminalPanel}>
        <TerminalWrapper activeLab={activeLab} />
      </div>
    </div>
  );
};

// Built-in Dark Mode styling for a professional Cyber Range look
const styles = {
  container: {
    display: "flex",
    flexDirection: "row",
    height: "100vh",
    backgroundColor: "#0d1117", // GitHub Dark background
    color: "#c9d1d9",
    fontFamily: "Segoe UI, Tahoma, Geneva, Verdana, sans-serif",
  },
  taskPanel: {
    flex: "0 0 400px", // תופס בדיוק 400 פיקסלים במקום חצי מסך, משאיר יותר מקום לטרמינל
    padding: "40px",
    borderRight: "1px solid #30363d",
    backgroundColor: "#0d1117",
    display: "flex",
    flexDirection: "column",
    gap: "20px",
    overflowY: "auto",
    textAlign: "left", // התיקון הקריטי: מיישר את כל הטקסט לשמאל!
  },
  terminalPanel: {
    flex: 1, // הטרמינל ייקח עכשיו את כל שאר המקום שנשאר במסך
    backgroundColor: "#000000",
    position: "relative",
    padding: "10px", // נותן לטרמינל קצת שוליים שחורים כדי שלא ייצמד לקירות
  },
  header: {
    borderBottom: "1px solid #30363d",
    paddingBottom: "15px",
    marginBottom: "10px",
  },
  badge: {
    backgroundColor: "#238636",
    color: "white",
    padding: "4px 8px",
    borderRadius: "4px",
    fontSize: "12px",
    fontWeight: "bold",
    display: "inline-block",
    marginTop: "10px",
  },
  description: {
    fontSize: "16px",
    lineHeight: "1.5",
  },
  taskBox: {
    backgroundColor: "#161b22",
    padding: "25px",
    borderRadius: "8px",
    border: "1px solid #30363d",
    marginTop: "20px",
  },
  input: {
    width: "90%",
    padding: "12px",
    marginTop: "15px",
    backgroundColor: "#0d1117",
    border: "1px solid #30363d",
    color: "#c9d1d9",
    borderRadius: "6px",
    fontSize: "16px",
  },
  button: {
    marginTop: "15px",
    padding: "12px 24px",
    backgroundColor: "#238636",
    color: "white",
    border: "none",
    borderRadius: "6px",
    cursor: "pointer",
    fontWeight: "bold",
    fontSize: "16px",
    display: "block",
  },

  terminateBtn: {
    backgroundColor: "transparent",
    color: "#f85149",
    border: "1px solid #f85149",
    padding: "8px 16px",
    borderRadius: "6px",
    cursor: "pointer",
    fontWeight: "bold",
    fontSize: "14px",
    transition: "all 0.2s",
    display: "flex",
    alignItems: "center",
    gap: "5px",
  },
};

export default LabWorkspace;
