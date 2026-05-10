use crate::{ai_proxy::_9router::_9RouterAIProxy, config::ai_proxy::AIProxyConfig};

pub trait AIProxy {
    fn is_install(&self) -> bool;
    fn is_running(&self) -> bool;
    fn install(&self) -> Result<(), String>;
    fn start(&self) -> Result<(), String>;
    fn stop(&self) -> Result<(), String>;
    fn get_url(&self) -> String;
}

pub fn get_proxy(name: &str, proxy_config: &AIProxyConfig) -> Option<Box<dyn AIProxy>> {
    match name {
        "9router" => {
            return Some(Box::new(_9RouterAIProxy {
                port: proxy_config.port,
            }));
        }

        _ => None,
    }
}
