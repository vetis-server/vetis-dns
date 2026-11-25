use crate::HookConfig;
use cel::Context;
use vetis::{
    Request, Response, VetisFutureResult,
    hook::{Hook, HookConfig as _, HookPhase},
    matcher::Matcher,
};

/// Proxy path
pub struct DnsHook {
    _config: HookConfig,
    matcher: Option<Box<dyn Matcher>>,
}

unsafe impl Send for DnsHook {}
unsafe impl Sync for DnsHook {}

impl DnsHook {
    /// Create a new dns hook with provided configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The dns hook configuration
    ///
    /// # Returns
    ///
    /// * `DnsHook` - The dns hook
    pub fn new(config: HookConfig) -> Self {
        let matcher = if let Some(matcher_config) = config.matcher() {
            Some(matcher_config.boxed_matcher())
        } else {
            None
        };

        DnsHook { matcher, _config: config }
    }
}

impl Hook for DnsHook {
    fn setup<'a>(&'a mut self) -> VetisFutureResult<'a, ()> {
        Box::pin(async move { Ok(()) })
    }

    fn has_matches(&self, context: &Context) -> bool {
        if let Some(matcher) = &self.matcher { matcher.is_match(context) } else { true }
    }

    fn phase(&self) -> HookPhase {
        HookPhase::Rewrite
    }

    fn matcher_mut(&mut self) -> Option<&mut Box<dyn Matcher>> {
        self.matcher
            .as_mut()
    }

    fn handle<'a>(
        &'a self,
        request: Request,
        _response: Option<Response>,
    ) -> VetisFutureResult<'a, (Request, Option<Response>)> {
        let future = async move { Ok((request, Some(Response::builder().empty()))) };
        Box::pin(future)
    }
}
