import React from "react";
import { Link, useLocation } from "react-router-dom";

const Navbar = () => {
  const location = useLocation();

  return (
    <nav style={styles.nav}>
      <div style={styles.logoContainer}>
        {/* לוגו האתר */}
        <h2 style={styles.logo}>
          <span style={styles.accent}>Cyber</span>Range
        </h2>
      </div>

      <div style={styles.linksContainer}>
        <Link
          to="/"
          style={location.pathname === "/" ? styles.activeLink : styles.link}
        >
          Dashboard
        </Link>
        <Link
          to="/academy"
          style={
            location.pathname === "/academy" ? styles.activeLink : styles.link
          }
        >
          Academy
        </Link>
        <Link
          to="/machines"
          style={
            location.pathname === "/machines" ? styles.activeLink : styles.link
          }
        >
          Machines
        </Link>
        <Link
          to="/leaderboard"
          style={
            location.pathname === "/leaderboard"
              ? styles.activeLink
              : styles.link
          }
        >
          Leaderboard
        </Link>
      </div>

      <div style={styles.userContainer}>
        <button style={styles.loginBtn}>Login</button>
      </div>
    </nav>
  );
};

const styles = {
  nav: {
    display: "flex",
    justifyContent: "space-between",
    alignItems: "center",
    backgroundColor: "#0d1117",
    padding: "0 30px",
    height: "70px",
    borderBottom: "1px solid #30363d",
    fontFamily: "Segoe UI, Tahoma, Geneva, Verdana, sans-serif",
  },
  logoContainer: {
    display: "flex",
    alignItems: "center",
  },
  logo: {
    color: "#c9d1d9",
    margin: 0,
    fontSize: "24px",
    letterSpacing: "1px",
  },
  accent: {
    color: "#238636",
  },
  linksContainer: {
    display: "flex",
    gap: "30px",
  },
  link: {
    color: "#8b949e",
    textDecoration: "none",
    fontSize: "16px",
    fontWeight: "500",
    transition: "color 0.2s",
  },
  activeLink: {
    color: "#ffffff",
    textDecoration: "none",
    fontSize: "16px",
    fontWeight: "bold",
    borderBottom: "2px solid #238636",
    paddingBottom: "22px", 
  },
  userContainer: {
    display: "flex",
    alignItems: "center",
  },
  loginBtn: {
    backgroundColor: "transparent",
    color: "#c9d1d9",
    border: "1px solid #c9d1d9",
    padding: "8px 20px",
    borderRadius: "6px",
    cursor: "pointer",
    fontWeight: "bold",
    fontSize: "14px",
    transition: "all 0.2s",
  },
};

export default Navbar;
