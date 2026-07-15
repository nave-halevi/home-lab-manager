import { useEffect, useRef } from "react";

import { FitAddon } from "@xterm/addon-fit";
import { Terminal } from "xterm";
import "xterm/css/xterm.css";

import { getTerminalUrl } from "../../labs/services/labService";

const TerminalWrapper = ({ activeLab }) => {
  const containerRef = useRef(null);

  useEffect(() => {
    if (!containerRef.current || !activeLab?.envId) {
      return undefined;
    }

    let isDisposed = false;
    let connectionFailed = false;

    const terminal = new Terminal({
      cursorBlink: true,
      fontFamily: "Courier New, monospace",
      fontSize: 15,
      scrollback: 5000,
      theme: {
        background: "#09090b",
        foreground: "#e4e4e7",
        cursor: "#fafafa",
      },
    });

    const fitAddon = new FitAddon();

    terminal.loadAddon(fitAddon);
    terminal.open(containerRef.current);
    fitAddon.fit();
    terminal.focus();

    terminal.writeln("\x1b[33mConnecting to the lab machine...\x1b[0m");

    const websocket = new WebSocket(getTerminalUrl(activeLab.envId));

    websocket.onopen = () => {
      if (isDisposed) {
        return;
      }

      terminal.writeln("\x1b[32mTerminal connected successfully.\x1b[0m");
    };

    websocket.onmessage = (event) => {
      if (isDisposed) {
        return;
      }

      terminal.write(event.data);
    };

    websocket.onerror = () => {
      if (isDisposed || connectionFailed) {
        return;
      }

      connectionFailed = true;

      terminal.writeln("\r\n\x1b[31mTerminal connection error.\x1b[0m");
    };

    websocket.onclose = () => {
      if (isDisposed) {
        return;
      }

      if (!connectionFailed) {
        terminal.writeln("\r\n\x1b[33mTerminal disconnected.\x1b[0m");
      }
    };

    const inputListener = terminal.onData((data) => {
      if (!isDisposed && websocket.readyState === WebSocket.OPEN) {
        websocket.send(data);
      }
    });

    const handleResize = () => {
      if (!isDisposed) {
        fitAddon.fit();
      }
    };

    window.addEventListener("resize", handleResize);

    return () => {
      isDisposed = true;

      window.removeEventListener("resize", handleResize);

      inputListener.dispose();

      websocket.onopen = null;
      websocket.onmessage = null;
      websocket.onerror = null;
      websocket.onclose = null;

      if (
        websocket.readyState === WebSocket.OPEN ||
        websocket.readyState === WebSocket.CONNECTING
      ) {
        websocket.close();
      }

      terminal.dispose();
    };
  }, [activeLab?.envId]);

  return (
    <div
      ref={containerRef}
      className="h-full min-h-[400px] w-full bg-zinc-950 p-2 text-left"
    />
  );
};

export default TerminalWrapper;
