import React, { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from '@tauri-apps/api/core';

// import { invoke } from '@tauri-apps/api/tauri'

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

  // Function to invoke the backend command
  const generateLogMessage = async () => {
    try {
      await invoke("generate_log"); // "generate_log" is the command that the backend will listen for
    } catch (error) {
      console.error("Failed to generate log:", error);
    }
  };

  return (
    <main className="p-4">
      <h1 className="text-primary text-2xl mb-4">Logs</h1>
      <div className="bg-gray-100 p-4 rounded">
        {logs.length > 0 ? (
          logs.map((log, index) => (
            <p key={index} className="text-primary">
              {log}
            </p>
          ))
        ) : (
          <p className="text-secondary">No logs received yet...</p>
        )}
      </div>
      <button
        className="mt-4 bg-blue-500 text-white p-2 rounded"
        onClick={generateLogMessage}
      >
        Generate log message in the backend
      </button>
    </main>
  );
};

export default Logs;
