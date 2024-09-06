pub trait Plugin: Send {
    fn name(&self) -> &str;

    fn status(&mut self) -> String {
        "n/a".to_owned()
    }

    fn show(&mut self) -> String {
        "n/a".to_owned()
    }

    fn send(&mut self, _action: &str, _data: &str) -> String {
        "n/a".to_owned()
    }

    fn action(&mut self, _action: &str, _data: &str, _data2: &str) -> String {
        "n/a".to_owned()
    }

    fn unload(&mut self) -> String {
        "n/a".to_owned()
    }
}

#[repr(C)]
pub struct PluginWrapper {
    pub plugin: Box<dyn Plugin + Send>,
}

impl PluginWrapper {
    pub fn new(plugin: Box<dyn Plugin + Send>) -> Self {
        PluginWrapper { plugin }
    }
}
