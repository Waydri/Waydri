use waydri_core::plugin::{Plugin, PluginContext, Capability};
use waydri_core::error::Result;

pub struct StatusBarPlugin {
    update_interval: f64,
    elapsed: f64,
    label: String,
}

impl StatusBarPlugin {
    pub fn new() -> Self {
        StatusBarPlugin {
            update_interval: 1.0,
            elapsed: 0.0,
            label: String::from("Waydri Bar"),
        }
    }
}

impl Plugin for StatusBarPlugin {
    fn name(&self) -> &str {
        "statusbar"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn on_start(&mut self, ctx: &mut PluginContext) -> Result<()> {
        let window_count = ctx.windows.count();
        self.label = format!("Waydri - {} windows", window_count);
        Ok(())
    }

    fn on_stop(&mut self) {
        self.elapsed = 0.0;
        self.label.clear();
    }

    fn on_tick(&mut self, dt: f64, ctx: &mut PluginContext) {
        self.elapsed += dt;
        if self.elapsed >= self.update_interval {
            self.elapsed = 0.0;
            let count = ctx.windows.count();
            let workspace = ctx.compositor.active_workspace();
            self.label = format!(
                "WS{} | {} windows",
                workspace + 1,
                count
            );
        }
    }

    fn capabilities(&self) -> Vec<Capability> {
        vec![
            Capability::Compositor,
            Capability::Rpc,
        ]
    }
}

#[no_mangle]
pub extern "C" fn create_plugin() -> Box<dyn Plugin> {
    Box::new(StatusBarPlugin::new())
}
