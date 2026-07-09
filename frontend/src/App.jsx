import { BrowserRouter, Routes, Route } from "react-router-dom";

import PublicLayout from "./layouts/PublicLayout";
import AppLayout from "./layouts/AppLayout";

import Landing from "./features/auth/pages/Landing";
import Login from "./features/auth/pages/Login";
import Register from "./features/auth/pages/Register";

import Dashboard from "./features/labs/pages/Dashboard";
import Academy from "./features/labs/pages/Academy";
import Machines from "./features/labs/pages/Machines";

import RequireAuth from "./routes/RequireAuth";

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

            <Route path="/academy" element={<Academy />} />

            <Route path="/machines" element={<Machines />} />

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
