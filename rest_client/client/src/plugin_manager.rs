use crate::errors::*;
use crate::request::Request;
use crate::request_response_plugin::RequestResponsePlugin;
use crate::response::Resp;
use plugin_framework::{declare_plugin, PluginManager};
use std::ffi::OsStr;

pub struct ClientPluginManager {
    plugin_manager: PluginManager,
    plugins: Vec<&'static dyn RequestResponsePlugin>,
}

impl ClientPluginManager {
    pub fn new() -> Self {
        Self {
            plugin_manager: PluginManager::new(),
            plugins: Vec::new(),
        }
    }

    pub unsafe fn load_plugin<P: AsRef<OsStr>>(&mut self, filename: P) -> Result<()> {
        self.plugin_manager
            .load_plugin(filename)
            .chain_err(|| "Unable to load plugin")?;
        let plugin = self.plugin_manager.plugins.last().unwrap();
        self.plugins.push(plugin);

        Ok(())
    }

    pub fn unload(&mut self) {
        self.plugin_manager.unload()
    }

    pub fn pre_send(&mut self, request: &mut Request) {
        debug!("Firing pre_send hooks");

        for plugin in &mut self.plugins {
            trace!("Firing pre_send for {:?}", plugin.name());
            plugin.pre_send(request);
        }
    }

    pub fn post_receive(&mut self, response: &mut Resp) {
        debug!("Firing post_receive hooks");

        for plugin in &mut self.plugins {
            trace!("Firing post_receive for {:?}", plugin.name());
            plugin.post_receive(response);
        }
    }
}
