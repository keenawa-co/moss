use anyhow::Result;
use hashbrown::HashMap;
use hecs::{Entity, World};
use hecs_hierarchy::Hierarchy;
use moss_ui::parts::{
    common::{ContextMenu, ContextMenuCell},
    toolbar::{ActivityCell, DescribeToolBarOutput, ToolBarLeftSide, ToolBarProjectCell},
};
use moss_uikit::component::{accessibility::*, layout::*, primitive::*};

pub trait AnyActionsContainer {
    const GROUP: &'static str;

    type ArchetypeMarker<Marker>;
    type DescribeOutput<Output>;

    fn describe<Output>(&self) -> Result<Self::DescribeOutput<Output>>;
}

type ActionGroupId = String;
type ActionGroup = HashMap<String, Entity>;

pub struct AccountActions(ActionGroup);

pub struct AccountActionsMarker;

pub struct DescribeAccountActionsOutput;

impl AnyActionsContainer for AccountActions {
    const GROUP: &'static str = "workbench/account";

    type ArchetypeMarker<Marker> = AccountActionsMarker;
    type DescribeOutput<Output> = DescribeAccountActionsOutput;

    fn describe<Output>(&self) -> Result<Self::DescribeOutput<Output>> {
        todo!()
    }
}

pub struct LayoutActions(ActionGroup);

type WidgetId = String;

pub struct ToolBarPart {
    widgets: HashMap<WidgetId, Entity>,
    actions: HashMap<ActionGroupId, ActionGroup>,
}

// pub fn describe_toolbar(frame: &Frame, project_menu: &Entity) -> Result<DescribeToolBarOutput> {
//     let result = DescribeToolBarOutput {
//         left_side: ToolBarLeftSide {
//             project: describe_project_cell(frame, project_menu)?,
//             activities: vec![ActivityCell {
//                 title: Some("Discovery"),
//                 tooltip: None,
//                 order: Some(Order(1)),
//                 icon: None,
//                 visibility: Visibility::Visible,
//                 nested: None,
//             }],
//         },
//     };

//     Ok(result)
// }

// fn describe_project_cell(frame: &Frame, entity: &Entity) -> Result<ToolBarProjectCell> {
//     let mut menu = ContextMenu {
//         content: Vec::new(),
//     };

//     for child in frame.children::<ToolBarProjectContextMenuMarker>(*entity) {
//         let text = frame.get::<&Text>(child)?;
//         let action = frame.get::<&Action>(child)?;

//         menu.content.push(ContextMenuCell {
//             action: Some((*action).clone()),
//             icon: None,
//             text: (*text).0,
//             shortcut: None,
//             nested: None,
//         });
//     }

//     Ok(ToolBarProjectCell { menu })
// }
