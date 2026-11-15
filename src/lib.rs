#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use crate::tokio::DnsHook;
use compact_str::ToCompactString;
use serde::{Deserialize, Serialize};
use vetis::{
    VetisHook, VetisResult,
    errors::{ConfigError, VetisError},
    hook::HookConfig,
    matcher::MatcherConfig,
};

#[cfg(feature = "runtime-tokio")]
/// Tokio runtime support module
pub mod tokio;

#[cfg(test)]
mod tests;

/// Builder for creating `DnsHookConfigBuilder` instances.
#[derive(Deserialize)]
pub struct DnsHookConfigBuilder {
    matcher: Option<Box<dyn MatcherConfig>>,
    target: String,
}

impl DnsHookConfigBuilder {
    /// Allow set a matcher
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn add_matcher<M>(mut self, matcher: M) -> Self
    where
        M: MatcherConfig + Send + Sync + 'static,
    {
        self.matcher = Some(Box::new(matcher));
        self
    }

    /// Allow set the target of the proxy path.
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn target(mut self, target: &str) -> Self {
        self.target = target.to_string();
        self
    }

    /// Build the `DnsHookConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `VetisResult<DnsHookConfig>` - The `DnsHookConfig` with the configured settings.
    pub fn build(self) -> VetisResult<DnsHookConfig> {
        if self
            .target
            .is_empty()
        {
            return Err(VetisError::Config(ConfigError::Hook(
                "Target cannot be empty".to_compact_string(),
            )));
        }

        Ok(DnsHookConfig {
            matcher: self
                .matcher
                .map_or(None, |val| Some(val)),
            target: self.target,
        })
    }
}

/// Configuration for a proxy path.
#[derive(Clone, Serialize, Deserialize)]
pub struct DnsHookConfig {
    matcher: Option<Box<dyn MatcherConfig>>,
    target: String,
    // TODO: Add custom proxy rules
    // TODO: Add support for custom headers
}

#[typetag::serde(name = "dns")]
impl HookConfig for DnsHookConfig {
    fn matcher(&self) -> Option<&Box<dyn MatcherConfig>> {
        self.matcher
            .as_ref()
    }

    fn boxed_clone(&self) -> Box<dyn HookConfig> {
        Box::new(self.clone())
    }

    fn boxed_hook(&self) -> Box<dyn VetisHook> {
        Box::new(DnsHook::new(self.clone()))
    }
}

impl DnsHookConfig {
    /// Creates a new `DnsHookConfigBuilder` with default settings.
    ///
    /// # Returns
    ///
    /// * `DnsHookConfigBuilder` - The builder.
    pub fn builder() -> DnsHookConfigBuilder {
        DnsHookConfigBuilder { matcher: None, target: "http://localhost:8080".to_string() }
    }

    /// Returns the target of the proxy path.
    ///
    /// # Returns
    ///
    /// * `&str` - The target of the proxy path.
    pub fn target(&self) -> &str {
        &self.target
    }
}
