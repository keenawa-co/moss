import React, { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

import { invokeTauriIpc } from "@/lib/backend/tauri";
import { Icon, Tooltip } from "@repo/moss-ui";
import { listen } from "@tauri-apps/api/event";

export type DescribeActivityOutput = { tooltip: string; order: number };

export const Home: React.FC = () => {
  const { t } = useTranslation(["ns1", "ns2"]);

  const handleNewWindowButton = async () => {
    const response = await invokeTauriIpc("create_new_window");
    console.log(response);
  };

  return (
    <div>
      <h1 className="text-[var(--moss-color-primary)]">{t("title")}</h1>

      <button className="bg-green-500 px-3" onClick={handleNewWindowButton}>
        New Window
      </button>

      <div>
        <Tooltip label="Test" className="text-[var(--moss-color-primary)]">
          <Icon icon="Code" />
        </Tooltip>
      </div>

      <SessionComponent />

      <div className="flex">
        <Icon icon="Accessibility" className="text-6xl hover:*:fill-green-500" />
        <Icon icon="NewProject" className="text-red-700 text-6xl hover:fill-green-500" />
      </div>

      <div className="mt-5">
        <div>List of 50 elements:</div>
        {Array.from({ length: 50 }).map((_, i) => (
          <div key={i}>{i + 1 === 50 ? "last element" : `${i + 1}: ${Math.random().toFixed(2)}`}</div>
        ))}
      </div>
    </div>
  );
};

const SessionComponent = () => {
  const { t } = useTranslation(["ns1", "ns2"]);
  const [data, setData] = useState<number | null>(null);

  const getAllActivities = async () => {
    try {
      console.log((await invokeTauriIpc("get_menu_items_by_namespace", { namespace: "headItem" })) as object);
    } catch (err) {
      console.error("Failed to get workbench state:", err);
    }
  };

  useEffect(() => {
    const unlisten = listen<number>("data-stream", (event) => {
      setData(event.payload);
    });

    getAllActivities();

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return (
    <>
      <span className="text-[var(--moss-color-primary)]">{t("description.part1")}</span>
      <br />
      <span className="bg-secondary text-[var(--moss-color-primary)]">{t("description.part1", { ns: "ns2" })}</span>
      {data !== null && <p>Received data: {data}</p>}
    </>
  );
};
