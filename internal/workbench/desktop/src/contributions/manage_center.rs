use crate::Contribution;

pub(crate) struct ManageCenterContribution;
impl Contribution for ManageCenterContribution {
    fn contribute(registry: &mut crate::RegistryManager) -> anyhow::Result<()> {
        /* ---------- Menus contributions ---------- */

        // let mut menus_registry_lock = registry.menus.write();

        // drop(menus_registry_lock);

        Ok(())
    }
}
