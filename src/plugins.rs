//! Lightweight internal plugin registry for command groups.

use crate::{
    commands::{anime, clear, manga, nasa, ping, utility},
    types::{Data, Error},
};
use poise::Command;

/// A command plugin that contributes one or more slash commands.
pub trait Plugin: Send + Sync {
    /// Stable plugin name used for diagnostics and tests.
    fn name(&self) -> &'static str;

    /// Returns the commands exposed by this plugin.
    fn commands(&self) -> Vec<Command<Data, Error>>;
}

/// Collection of registered plugins.
pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRegistry {
    /// Returns the registered plugin names.
    pub fn names(&self) -> Vec<&'static str> {
        self.plugins.iter().map(|plugin| plugin.name()).collect()
    }

    /// Flattens all plugin commands into a single list.
    pub fn commands(&self) -> Vec<Command<Data, Error>> {
        self.plugins
            .iter()
            .flat_map(|plugin| plugin.commands())
            .collect()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self {
            plugins: vec![
                Box::new(CorePlugin),
                Box::new(ContentPlugin),
                Box::new(UtilityPlugin),
            ],
        }
    }
}

/// Returns the default plugin registry.
pub fn registry() -> PluginRegistry {
    PluginRegistry::default()
}

struct CorePlugin;

impl Plugin for CorePlugin {
    fn name(&self) -> &'static str {
        "core"
    }

    fn commands(&self) -> Vec<Command<Data, Error>> {
        vec![ping::ping(), clear::clear()]
    }
}

struct ContentPlugin;

impl Plugin for ContentPlugin {
    fn name(&self) -> &'static str {
        "content"
    }

    fn commands(&self) -> Vec<Command<Data, Error>> {
        vec![nasa::apod(), anime::random(), manga::random()]
    }
}

struct UtilityPlugin;

impl Plugin for UtilityPlugin {
    fn name(&self) -> &'static str {
        "utility"
    }

    fn commands(&self) -> Vec<Command<Data, Error>> {
        vec![utility::about(), utility::uptime(), utility::stats()]
    }
}

#[cfg(test)]
mod tests {
    use super::registry;

    #[test]
    fn exposes_expected_plugins() {
        assert_eq!(registry().names(), vec!["core", "content", "utility"]);
    }
}
