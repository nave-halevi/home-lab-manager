import React from "react";
import { Link, useLocation, useNavigate } from "react-router-dom";

import Button from "../components/ui/Button";

const Navbar = () => {
  const location = useLocation();
  const navigate = useNavigate();

  const linkClass = (path) =>
    `transition-colors duration-200 font-medium pb-1 ${
      location.pathname === path
        ? "text-red-500 border-b-2 border-red-500"
        : "text-gray-400 hover:text-white"
    }`;

  return (
    <nav className="h-18 border-b border-zinc-800 bg-[#0a0a0a]">
      <div className="mx-auto flex h-full max-w-7xl items-center justify-between px-8">
        {/* Logo */}
        <div>
          <h2 className="select-none text-2xl font-bold tracking-wide text-white">
            <span className="text-red-600">Cyber</span>Range
          </h2>
        </div>

        <div className="flex items-center gap-8">
          <Link to="/" className={linkClass("/")}>
            Dashboard
          </Link>

          <Link to="/academy" className={linkClass("/academy")}>
            Academy
          </Link>

          <Link to="/machines" className={linkClass("/machines")}>
            Machines
          </Link>

          <Link to="/leaderboard" className={linkClass("/leaderboard")}>
            Leaderboard
          </Link>
        </div>

        <Button variant="primary" onClick={() => navigate("/login")}>
          Login
        </Button>
      </div>
    </nav>
  );
};

export default Navbar;
