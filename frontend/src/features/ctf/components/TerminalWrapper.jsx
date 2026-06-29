import React, { useEffect, useRef } from "react";
import { Terminal } from "xterm";
import { FitAddon } from "@xterm/addon-fit";
import "xterm/css/xterm.css";

const TerminalWrapper = ({ activeLab }) => {
  const containerRef = useRef(null);
  const terminalRef = useRef(null);
  const websocketRef = useRef(null);

  useEffect(() => {
    if (!containerRef.current || !activeLab) return;

    const term = new Terminal({
      cursorBlink: true,
      fontFamily: "Courier New, monospace",
      fontSize: 16,
      theme: {
        background: "#0f141c",
        foreground: "#00ff00",
        cursor: "#00ff00",
      },
    });

    const fitAddon = new FitAddon();
    term.loadAddon(fitAddon);
    term.open(containerRef.current);
    fitAddon.fit();

    term.writeln("====================================");
    term.writeln("   CONNECTING TO REMOTE INFRASTRUCTURE ");
    term.writeln("====================================");

    const wsUrl = `ws://localhost:3000/api/lab/terminal/${activeLab.port}`;
    const ws = new WebSocket(wsUrl);
    websocketRef.current = ws;

    ws.onopen = () => {
      term.writeln("Status: Connection established successfully.");
      term.writeln("------------------------------------");
    };

    ws.onmessage = (event) => {
      term.write(event.data);
    };

    ws.onclose = () => {
      term.writeln("");
      term.writeln("Status: Connection closed by remote host.");
    };

    ws.onerror = () => {
      term.writeln("");
      term.writeln("Status: Core network bridge error.");
    };

    const handleResize = () => {
      fitAddon.fit();
    };
    window.addEventListener("resize", handleResize);

    const dataListener = term.onData((data) => {
      if (ws.readyState === WebSocket.OPEN) {
        ws.send(data);
      }
    });

    terminalRef.current = term;

    return () => {
      window.removeEventListener("resize", handleResize);
      dataListener.dispose();
      if (
        ws.readyState === WebSocket.OPEN ||
        ws.readyState === WebSocket.CONNECTING
      ) {
        ws.close();
      }
      term.dispose();
    };
  }, [activeLab]);

  return (
    <div
      ref={containerRef}
      style={{
        width: "100%",
        height: "100%",
        padding: "10px",
        boxSizing: "border-box",
        textAlign: "left",
      }}
    />
  );
};

export default TerminalWrapper;
