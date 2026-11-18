use crate::DnsHookConfig;
use cel::Context;
use vetis::{
    HookPhases, Request, Response, VetisFutureResult,
    hook::{Hook, HookConfig as _, HookPhase},
    make_bitflags,
    matcher::Matcher,
};

/// Proxy path
pub struct DnsHook {
    _config: DnsHookConfig,
    matcher: Option<Box<dyn Matcher>>,
}

unsafe impl Send for DnsHook {}
unsafe impl Sync for DnsHook {}

impl DnsHook {
    /// Create a new proxy path with provided configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The proxy path configuration
    ///
    /// # Returns
    ///
    /// * `ProxyHook` - The proxy path
    pub fn new(config: DnsHookConfig) -> DnsHook {
        let matcher = if let Some(matcher_config) = config.matcher() {
            Some(matcher_config.boxed_matcher())
        } else {
            None
        };

        DnsHook { matcher, _config: config }
    }
}

impl Hook for DnsHook {
    fn has_matches(&self, context: &Context) -> bool {
        if let Some(matcher) = &self.matcher { matcher.is_match(context) } else { true }
    }

    fn phases(&self) -> HookPhases {
        make_bitflags!(HookPhase::Rewrite)
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
