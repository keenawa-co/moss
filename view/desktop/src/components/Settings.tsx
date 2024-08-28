import React, { useState, useEffect } from "react";
import { useTranslation } from "react-i18next";
import { commands } from "@/bindings";
import { listen } from "@tauri-apps/api/event";
import { LanguageSelector, ThemeSelector } from "@/components";

export const Settings: React.FC = () => {
  const { t } = useTranslation(["ns1", "ns2"]);

  const [number, setNumber] = useState<number>(0);
  const [constantValue, setConstantValue] = useState<number>(0);

  useEffect(() => {
    const unlisten = listen<number>("font-size-update-event", (event) => {
      setConstantValue(event.payload);
    });

    return () => {
      unlisten.then((unlistenFn) => unlistenFn());
    };
  }, []);

  const handleButtonClick = async () => {
    try {
      const response = await commands.updateFontSize();
      console.log(response);
      if (response.status === "ok") {
        console.log("OK");
      }
    } catch (err) {
      console.error("Failed to update font size:", err);
    }
  };

  return (
    <main>
      <div>
        <h3>{t("selectLanguage")}</h3>
        <LanguageSelector />
      </div>
      <div>
        <h3>{t("selectTheme")}</h3>
        <ThemeSelector />
      </div>
      <div>
        <h3>Update Font Size</h3>
        <input type="number" value={number} onChange={(e) => setNumber(parseInt(e.target.value))} />
        <button onClick={handleButtonClick}>Update</button>
      </div>
      <div>
        <h3>Font Size</h3>
        <p>{constantValue}</p>
      </div>
    </main>
  );
};

export default Settings;
