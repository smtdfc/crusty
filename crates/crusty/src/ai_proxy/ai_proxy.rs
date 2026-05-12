use crate::{
    ai_proxy::_9router::_9RouterAIProxy, config::ai_proxy::AIProxyConfig,
    exceptions::crusty::CrustyError,
};

pub trait AIProxy {
    fn is_install(&self) -> Result<bool, CrustyError>;
    fn is_running(&self) -> Result<bool, CrustyError>;
    fn install(&self) -> Result<(), CrustyError>;
    fn start(&self) -> Result<(), CrustyError>;
    fn stop(&self) -> Result<(), CrustyError>;
    fn get_url(&self) -> String;
    fn get_dashboard_url(&self) -> String;
}

pub fn get_proxy(name: &str, proxy_config: &AIProxyConfig) -> Option<Box<dyn AIProxy>> {
    match name {
        "9router" => {
            return Some(Box::new(_9RouterAIProxy {
                is_local: proxy_config.is_local,
                host: proxy_config.host.clone(),
                port: proxy_config.port,
            }));
        }

        _ => None,
    }
}
