import React, { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";

export const Logs: React.FC = () => {
  const [logs, setLogs] = useState<string[]>([]);

  useEffect(() => {
    // Listen for logs from the backend
    const unlisten = listen<string>("logs-stream", (event) => {
      setLogs((prevLogs) => [...prevLogs, event.payload]);
    });

    // Cleanup the listener on component unmount
    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return (
    <main className="p-4">
      <h1 className="text-[rgba(var(--color-primary))] text-2xl mb-4">Logs</h1>
      <div className="bg-gray-100 p-4 rounded">
        {logs.length > 0 ? (
          logs.map((log, index) => (
            <p key={index} className="text-[rgba(var(--color-primary))]">
              {log}
            </p>
          ))
        ) : (
          <p className="text-secondary">No logs received yet...</p>
        )}
      </div>
    </main>
  );
};
