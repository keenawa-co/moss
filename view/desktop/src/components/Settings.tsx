import { useTranslation } from "react-i18next";
import { commands, SessionInfoDTO } from "@/bindings";
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
} from "@repo/ui";

// import { commands } from '@/bindings'

/*
export const Settings = () => {
  const { t } = useTranslation();

  return (
    <main>
      <h1>{t("settings")}</h1>
      <span>{t("user", { name: "Jevgenijs 🦇" })}</span>
    </main>
  );
};
*/

export const Settings: React.FC = () => {
  const { t } = useTranslation(["ns1", "ns2"]);

  return (
    <main>
      <span>{t("description.part1")}</span>
      <span>{t("description.part1", { ns: "ns2" })}</span>
    </main>
  );
};

export default Settings;
