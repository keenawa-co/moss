use anyhow::Result;
use hashbrown::HashMap;
use moss_hecs::{EntityBuilder, Frame};
use moss_hecs_hierarchy::HierarchyMut;
use moss_uikit::component::{accessibility::Action, layout::Group, primitive::Text};

pub struct ContextMenuItem {
    text: String,
    tooltip: String,
}

pub struct ContextMenu(HashMap<Option<String>, Vec<ContextMenuItem>>);

impl ContextMenu {
    pub fn insert(&mut self, group: Option<String>, item: ContextMenuItem) {
        let group = self.0.get_mut(&group).unwrap();
        group.push(item);
    }
}

pub struct ManageContextMenuMarker;
pub struct ManageContextMenuGroupMarker;

fn manage(frame: &mut Frame) -> Result<()> {
    let manage_entity = {
        let mut this = EntityBuilder::new();

        frame.spawn(this.build())
    };

    let manage_entity_general_group = {
        let mut this = EntityBuilder::new();
        this.add(Group {
            id: "general".to_string(),
            order: 1,
        });

        frame.spawn(this.build())
    };

    let settings_context_menu_item = {
        let mut this = EntityBuilder::new();
        this.add(Text("Settings"));
        this.add(Action("workbench.action.manage.openSettings"));

        frame.spawn(this.build())
    };

    let shortcuts_context_menu_item = {
        let mut this = EntityBuilder::new();
        this.add(Text("Shortcuts"));
        this.add(Action("workbench.action.manage.openShortcuts"));

        frame.spawn(this.build())
    };

    frame.attach::<ManageContextMenuMarker>(manage_entity_general_group, manage_entity)?;

    frame.attach::<ManageContextMenuGroupMarker>(
        settings_context_menu_item,
        manage_entity_general_group,
    )?;
    frame.attach::<ManageContextMenuGroupMarker>(
        shortcuts_context_menu_item,
        manage_entity_general_group,
    )?;

    Ok(())
}
