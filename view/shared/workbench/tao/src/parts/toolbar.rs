use anyhow::Result;
use moss_hecs::{Entity, Frame};
use moss_hecs_hierarchy::Hierarchy;
use moss_ui::parts::{
    common::{ContextMenu, ContextMenuCell},
    toolbar::{ActivityCell, DescribeToolBarOutput, ToolBarLeftSide, ToolBarProjectCell},
};
use moss_uikit::component::{accessibility::*, layout::*, primitive::*};

use crate::ToolBarProjectContextMenuMarker;

pub mod describe;

pub fn describe_toolbar(frame: &Frame, project_menu: &Entity) -> Result<DescribeToolBarOutput> {
    let result = DescribeToolBarOutput {
        left_side: ToolBarLeftSide {
            project: describe_project_cell(frame, project_menu)?,
            activities: vec![ActivityCell {
                title: Some("Discovery"),
                tooltip: None,
                order: Some(Order { value: 1 }),
                icon: None,
                visibility: Visibility::Visible,
                nested: None,
            }],
        },
    };

    Ok(result)
}

fn describe_project_cell(frame: &Frame, entity: &Entity) -> Result<ToolBarProjectCell> {
    let mut menu = ContextMenu {
        content: Vec::new(),
    };

    for child in frame.children::<ToolBarProjectContextMenuMarker>(*entity) {
        let text = frame.get::<&Text>(child)?;
        let action = frame.get::<&Action>(child)?;

        menu.content.push(ContextMenuCell {
            action: Some((*action).clone()),
            icon: None,
            text: (*text).0,
            shortcut: None,
            nested: None,
        });
    }

    Ok(ToolBarProjectCell { menu })
}
