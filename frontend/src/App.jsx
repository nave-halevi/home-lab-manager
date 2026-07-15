import { BrowserRouter, Routes, Route } from "react-router-dom";

import PublicLayout from "./layouts/PublicLayout";
import AppLayout from "./layouts/AppLayout";
import Landing from "./features/auth/pages/Landing";
import Login from "./features/auth/pages/Login";
import Register from "./features/auth/pages/Register";
import Dashboard from "./features/dashboard/pages/Dashboard";
import Machines from "./features/labs/pages/Machines";
import CoursesPage from "./features/academy/pages/CoursesPage";
import CoursePage from "./features/academy/pages/CoursePage";
import ProfilePage from "./features/profile/pages/ProfilePage";
import RequireAuth from "./routes/RequireAuth";
import RequireAdmin from "./routes/RequireAdmin";
import AdminLayout from "./features/admin/components/AdminLayout";
import AdminOverviewPage from "./features/admin/pages/AdminOverviewPage";
import AdminCoursesPage from "./features/admin/pages/AdminCoursesPage";
import AdminCourseEditorPage from "./features/admin/pages/AdminCourseEditorPage";
import AdminScenariosPage from "./features/admin/pages/AdminScenariosPage";
import AdminUsersPage from "./features/admin/pages/AdminUsersPage";
import AdminUserDetailsPage from "./features/admin/pages/AdminUserDetailsPage";
import AdminLabsPage from "./features/admin/pages/AdminLabsPage";
import AdminFlagsPage from "./features/admin/pages/AdminFlagsPage";
import AdminActivityPage from "./features/admin/pages/AdminActivityPage";

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
            <Route path="/academy" element={<CoursesPage />} />
            <Route path="/academy/:courseId" element={<CoursePage />} />
            <Route path="/machines" element={<Machines />} />
            <Route path="/profile" element={<ProfilePage />} />

            <Route element={<RequireAdmin />}>
              <Route path="/admin" element={<AdminLayout />}>
                <Route index element={<AdminOverviewPage />} />
                <Route path="academy" element={<AdminCoursesPage />} />
                <Route
                  path="academy/courses/:courseId"
                  element={<AdminCourseEditorPage />}
                />
                <Route path="scenarios" element={<AdminScenariosPage />} />
                <Route path="users" element={<AdminUsersPage />} />
                <Route path="users/:userId" element={<AdminUserDetailsPage />} />
                <Route path="labs" element={<AdminLabsPage />} />
                <Route path="flags" element={<AdminFlagsPage />} />
                <Route path="activity" element={<AdminActivityPage />} />
              </Route>
            </Route>

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
