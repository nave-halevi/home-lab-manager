import React from "react";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Navbar from "./components/Navbar";
import LabsPage from "./pages/LabsPage";

function App() {
  return (
    <Router>
      <div
        className="App"
        style={{ backgroundColor: "#0d1117", minHeight: "100vh" }}
      >
        <Navbar />

        <Routes>
          <Route path="/" element={<LabsPage />} />

          <Route
            path="/academy"
            element={
              <div
                style={{ color: "white", padding: "50px", textAlign: "center" }}
              >
                Academy Paths Coming Soon...
              </div>
            }
          />
          <Route
            path="/machines"
            element={
              <div
                style={{ color: "white", padding: "50px", textAlign: "center" }}
              >
                Standalone Machines Coming Soon...
              </div>
            }
          />
          <Route
            path="/leaderboard"
            element={
              <div
                style={{ color: "white", padding: "50px", textAlign: "center" }}
              >
                Leaderboard Coming Soon...
              </div>
            }
          />
        </Routes>
      </div>
    </Router>
  );
}

export default App;
