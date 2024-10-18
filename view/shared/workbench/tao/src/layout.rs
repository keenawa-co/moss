use anyhow::Result;
use hashbrown::HashMap;
use hecs::{DynamicBundle, Entity, EntityBuilder, World as Registry};
use std::any::{Any, TypeId};

use crate::{
    contribution,
    parts::{AnyPart, PartId},
};

pub(crate) struct PartMeta {}

pub struct Layout {
    pub(crate) registry: Registry,
    pub(crate) tree_view_container_groups: HashMap<String, Vec<Entity>>,
    pub(crate) tree_views: HashMap<String, Vec<Entity>>,
}

impl Layout {
    pub(crate) fn new() -> Self {
        Self {
            registry: Registry::new(),
            tree_view_container_groups: HashMap::new(),
            tree_views: HashMap::new(),
        }
    }
}

impl Layout {
    pub(crate) fn contribute(&mut self, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {
        f(self)
    }

    pub(crate) fn add_tree_view_container(
        &mut self,
        group_id: &str,
        view_container_id: &'static str,
        bundle: impl DynamicBundle,
    ) {
        let mut entity_builder = EntityBuilder::new();
        entity_builder.add(view_container_id).add_bundle(bundle);
        let entity = self.registry.spawn(entity_builder.build());

        if let Some(group) = self.tree_view_container_groups.get_mut(group_id) {
            group.push(entity);
        } else {
            self.tree_view_container_groups
                .insert(group_id.to_string(), vec![entity]);
        }

        self.tree_views
            .insert(view_container_id.to_string(), Vec::new());
    }

    pub(crate) fn add_tree_view(
        &mut self,
        view_container_id: &str,
        bundle: impl DynamicBundle,
    ) -> Result<()> {
        if let Some(container) = self.tree_views.get_mut(view_container_id) {
            let mut entity_builder = EntityBuilder::new();
            entity_builder.add_bundle(bundle);

            container.push(self.registry.spawn(entity_builder.build()));

            Ok(())
        } else {
            Err(anyhow!("{view_container_id} view container is undefined"))
        }
    }
}
