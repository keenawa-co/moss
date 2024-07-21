pub mod configuration_editing;

use configuration::common::configuration_registry::ConfigurationScope;

lazy_static! {
    static ref APPLICATION_SCOPE_RANGE: [ConfigurationScope; 1] = [ConfigurationScope::Application];
    static ref MACHINE_SCOPE_RANGE: [ConfigurationScope; 1] = [ConfigurationScope::Application];
}
