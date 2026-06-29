import { BrowserRouter, Routes, Route } from "react-router-dom";

import PublicLayout from "./layouts/PublicLayout";
import AppLayout from "./layouts/AppLayout";

import Landing from "./pages/Landing";
import Login from "./pages/Login";
import Register from "./pages/Register";
import LabsPage from "./pages/LabsPage";

import RequireAuth from "./routes/RequireAuth";
import Dashboard from "./pages/Dashboard";
import Academy from "./pages/Academy";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        {/* PUBLIC ROUTES */}
        <Route element={<PublicLayout />}>
          <Route path="/" element={<Landing />} />
          <Route path="/login" element={<Login />} />
          <Route path="/register" element={<Register />} />
        </Route>

        {/* PROTECTED ROUTES */}
        <Route element={<RequireAuth />}>
          <Route element={<AppLayout />}>
            <Route path="/dashboard" element={<Dashboard />} />

            <Route
              path="/academy"
              element={<Academy />}
            />

            <Route
              path="/machines"
              element={
                <div className="p-10 text-white">Machines Coming Soon...</div>
              }
            />

            <Route
              path="/leaderboard"
              element={
                <div className="p-10 text-white">
                  Leaderboard Coming Soon...
                </div>
              }
            />
          </Route>
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
