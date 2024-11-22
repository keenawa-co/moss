import { useTranslation } from "react-i18next";
import React, { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { Tooltip, DropdownMenu, Icon } from "@repo/ui";
import { invokeIpc } from "@/lib/backend/tauri";
import { useStoredString, useUpdateStoredString } from "@/hooks/useReactQuery";

export type DescribeActivityOutput = { tooltip: string; order: number };

const SessionComponent = () => {
  const { t } = useTranslation(["ns1", "ns2"]);
  const [data, setData] = useState<number | null>(null);

  let getAllActivities = async () => {
    try {
      // console.log((await invokeIpc("get_view_content")) as object);
      console.log((await invokeIpc("get_menu_items_by_namespace", { namespace: "headItem" })) as object);
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
      <span className="text-[rgba(var(--color-primary))]">{t("description.part1")}</span>
      <br />
      <span className="bg-secondary text-[rgba(var(--color-primary))]">{t("description.part1", { ns: "ns2" })}</span>
      {data !== null && <p>Received data: {data}</p>}
    </>
  );
};

export const Home: React.FC = () => {
  const { t } = useTranslation(["ns1", "ns2"]);

  const handleNewWindowButton = async () => {
    const response = await invokeIpc("create_new_window");
    console.log(response);
  };

  return (
    <div>
      <h1 className="text-[rgba(var(--color-primary))]">{t("title")}</h1>

      <button className="bg-green-500 px-3" onClick={handleNewWindowButton}>
        New Window
      </button>
      <StoredStringUpdater />
      <div>
        <Tooltip label="Test" className="text-[rgba(var(--color-primary))]">
          <Icon icon="Code" />
        </Tooltip>
      </div>
      <SessionComponent />
      <div>
        {/* <DropdownMenu>
          <DropdownMenuTrigger className="text-[rgba(var(--color-primary))]">Click me!</DropdownMenuTrigger>

          <DropdownMenuContent>
            <DropdownMenuItem icon="Search">Menu item 1</DropdownMenuItem>
            <DropdownMenuItem>
              <DropdownMenuLabel>Menu item 2</DropdownMenuLabel>
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu> */}
      </div>

      <div className="flex">
        <Icon icon="Accessibility" className="text-6xl hover:*:fill-green-500" />
        <Icon icon="NewProject" className="text-red-700 text-6xl hover:fill-green-500" />
      </div>
      {/*
      <div className="w-96 bg-red-600">
        {new Array(77).fill(0).map((_, index) => (
          <div key={index}>
            {index} - {Math.random()}
          </div>
        ))}
        <div> last element</div>
      </div> */}
    </div>
  );
};

const StoredStringUpdater: React.FC = () => {
  const [newString, setNewString] = useState<string>("");
  const mutation = useUpdateStoredString();
  const { data: storedString } = useStoredString();

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    mutation.mutate(newString);
  };

  const handleLogCurrentString = async () => {
    console.log("Current Stored String:", storedString);
  };

  return (
    <div>
      <hr />
      <h2>Update String:</h2>
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          value={newString}
          onChange={(e) => setNewString(e.target.value)}
          placeholder="Enter new string"
          required
        />
        <button type="submit" disabled={mutation.isPending}>
          {mutation.isPending ? "Updating..." : "Update"}
        </button>
      </form>
      {mutation.isError && <p style={{ color: "red" }}>Error: {mutation.error?.message}</p>}
      {mutation.isSuccess && <p style={{ color: "green" }}>String successfully updated!</p>}

      <div>
        currentString: <span className="font-extrabold">{storedString}</span>
      </div>
      <hr />
    </div>
  );
};
