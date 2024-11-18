// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
//
// The necessary import statements have been automatically added by a Python script.
// This ensures that all required dependencies are correctly referenced and available
// within this module.
//
// If you need to add or modify imports, please update the import_map in the script and
// re-run `make gen-models` it to regenerate the file accordingly.

import type { LocalizedString } from "@repo/moss-str";

export type Action = string;

export type ActionMenuItem = {
  command: CommandAction;
  group: MenuGroup | null;
  order: bigint | null;
  when: string | null;
  visibility: MenuItemVisibility;
};

export type CommandAction = {
  id: string;
  title: LocalizedString | null;
  tooltip: string | null;
  description: LocalizedString | null;
  icon: string | null;
  toggled: CommandActionToggle | null;
};

export type CommandActionToggle = {
  condition: string;
  icon: string | null;
  tooltip: string | null;
  title: LocalizedString | null;
};

export type MenuGroup = { id: string; order: bigint | null; description: LocalizedString | null };

export type MenuItem = { action: ActionMenuItem } | { submenu: SubmenuMenuItem };

export type MenuItemVisibility = "classic" | "hidden" | "compact";

export type SubmenuMenuItem = {
  submenuId: string;
  defaultActionId: string | null;
  title: LocalizedString | null;
  group: MenuGroup | null;
  order: bigint | null;
  when: string | null;
  visibility: MenuItemVisibility;
};
