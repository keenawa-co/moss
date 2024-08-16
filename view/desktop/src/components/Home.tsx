import { useTranslation } from "react-i18next";
import { commands, SessionInfoDTO } from "../bindings";
import React, { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import {
  Tooltip,
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  Icon,
} from "../../../shared/ui/src";

import { Typescript, Acc } from "../../../shared/icons/build";

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
          <Icon icon="Code" />
        </Tooltip>
      </div>

      <div>
        <DropdownMenu>
          <DropdownMenuTrigger>Click me!</DropdownMenuTrigger>

          <DropdownMenuContent>
            <DropdownMenuItem icon="Search">Menu item 1</DropdownMenuItem>
            <DropdownMenuItem>
              <DropdownMenuLabel>Menu item 2</DropdownMenuLabel>
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>

      <div className="flex">
        <Icon icon="Accessibility" className="text-6xl hover:*:fill-green-500" />
        <Icon icon="NewProject" className="text-6xl text-red-600 hover:fill-green-500" />
      </div>
    </main>
  );
};

export default Home;
