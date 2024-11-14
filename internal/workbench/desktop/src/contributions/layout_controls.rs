use moss_str::localize;

use crate::{
    menu::{ActionMenuItem, BuiltInMenuNamespaces, CommandAction, MenuItem},
    Contribution,
};

pub struct LayoutControlsContribution;
impl Contribution for LayoutControlsContribution {
    fn contribute(registry: &mut crate::RegistryManager) -> anyhow::Result<()> {
        /* ---------- Menus contributions ---------- */

        let mut menus_registry_lock = registry.menus.write();

        menus_registry_lock.append_menu_items(vec![
            //
            // Toggle Primary Side Bar
            //
            (
                BuiltInMenuNamespaces::ViewTitleContext.into(), // FIXME:
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "layoutControls.openPrimarySidebar".into(),
                        title: localize!("recents.togglePrimarySideBar", "Toggle Primary Side Bar"),
                        tooltip: None,
                        description: None,
                        icon: None,
                    },
                    group: None,
                    order: None,
                    when: None,
                    toggled: None,
                }),
            ),
            (
                BuiltInMenuNamespaces::ViewTitleContext.into(), // FIXME:
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "layoutControls.closePrimarySidebar".into(),
                        title: localize!("recents.togglePrimarySideBar", "Toggle Primary Side Bar"),
                        tooltip: None,
                        description: None,
                        icon: None,
                    },
                    group: None,
                    order: None,
                    when: None,
                    toggled: None,
                }),
            ),
            //
            // Toggle Secondary Side Bar
            //
            (
                BuiltInMenuNamespaces::ViewTitleContext.into(), // FIXME:
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "layoutControls.openSecondarySidebar".into(),
                        title: localize!(
                            "layoutControls.togglePrimarySideBar",
                            "Toggle Primary Side Bar"
                        ),
                        tooltip: None,
                        description: None,
                        icon: None,
                    },
                    group: None,
                    order: None,
                    when: None,
                    toggled: None,
                }),
            ),
            (
                BuiltInMenuNamespaces::ViewTitleContext.into(), // FIXME:
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "layoutControls.closeSecondarySidebar".into(),
                        title: localize!(
                            "layoutControls.togglePrimarySideBar",
                            "Toggle Primary Side Bar"
                        ),
                        tooltip: None,
                        description: None,
                        icon: None,
                    },
                    group: None,
                    order: None,
                    when: None,
                    toggled: None,
                }),
            ),
            //
            // Toggle Panel
            //
            (
                BuiltInMenuNamespaces::ViewTitleContext.into(), // FIXME:
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "layoutControls.openPanel".into(),
                        title: localize!("layoutControls.togglePanel", "Toggle Panel"),
                        tooltip: None,
                        description: None,
                        icon: None,
                    },
                    group: None,
                    order: None,
                    when: None,
                    toggled: None,
                }),
            ),
            (
                BuiltInMenuNamespaces::ViewTitleContext.into(), // FIXME:
                MenuItem::Action(ActionMenuItem {
                    command: CommandAction {
                        id: "layoutControls.closePanel".into(),
                        title: localize!("layoutControls.togglePanel", "Toggle Panel"),
                        tooltip: None,
                        description: None,
                        icon: None,
                    },
                    group: None,
                    order: None,
                    when: None,
                    toggled: None,
                }),
            ),
        ]);

        //
        // Customize Layout
        //
        // menus_registry_lock.append_menu_item(menu_id, item);

        drop(menus_registry_lock);

        Ok(())
    }
}
