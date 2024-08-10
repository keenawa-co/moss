import { useTranslation } from "react-i18next";
import { commands, SessionInfoDTO } from "../bindings";
import React, { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { Tooltip, CodeIcon } from "../../../shared/ui/src";

export const Home: React.FC = () => {
  const { t } = useTranslation(["ns1", "ns2"]);
  const [sessionInfo, setSessionInfo] = useState<SessionInfoDTO | null>(null);
  const [data, setData] = useState<number | null>(null);
  const [workbenchState, setWorkbenchState] = useState<string>("empty");

  let getWorkbenchState = async () => {
    try {
      const response = await commands.workbenchGetState();
      console.log(response);
      if (response.status === "ok") {
        setWorkbenchState(response.data);
      }
    } catch (err) {
      console.error("Failed to get workbench state:", err);
    }
  };

  useEffect(() => {
    const unlisten = listen<number>("data-stream", (event) => {
      setData(event.payload);
    });

    getWorkbenchState();

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  useEffect(() => {
    if (sessionInfo) {
      console.log("Session restored:", sessionInfo);
    }
  }, [sessionInfo]);

  const handleRestoreSession = async () => {
    try {
      let response = await commands.restoreSession(null);
      if (response.status === "ok") {
        setSessionInfo(response.data);
      }
    } catch (error) {
      console.error("Failed to restore session:", error);
    }
  };

  return (
    <main>
      <h1>{t("title")}</h1>

      {sessionInfo ? (
        <div>
          <p>Session: {sessionInfo.session.id}</p>
          <p>Project: {sessionInfo.project.source}</p>
        </div>
      ) : (
        <p>No session</p>
      )}

      <p>
        Workspace: <span className="bg-red-500 text-zinc-900"> {workbenchState}</span>
      </p>

      <button className="bg-red-500" onClick={handleRestoreSession}>
        Restore Session
      </button>

      <span>{t("description.part1")}</span>
      <span>{t("description.part1", { ns: "ns2" })}</span>
      {data !== null && <p>Received data: {data}</p>}

      <div>
        <Tooltip label="Test">
          <CodeIcon className="fill-red-500 w-4" />
        </Tooltip>
      </div>
    </main>
  );
};

export default Home;
