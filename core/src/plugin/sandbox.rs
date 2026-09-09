#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxCapability {
    Network,
    FilesystemRead,
    FilesystemWrite,
    CompositorControl,
    Input,
    Rpc,
    None,
}

impl SandboxCapability {
    pub fn from_str(value: &str) -> SandboxCapability {
        match value {
            "network" => SandboxCapability::Network,
            "fs_read" => SandboxCapability::FilesystemRead,
            "fs_write" => SandboxCapability::FilesystemWrite,
            "compositor" => SandboxCapability::CompositorControl,
            "input" => SandboxCapability::Input,
            "rpc" => SandboxCapability::Rpc,
            _ => SandboxCapability::None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            SandboxCapability::Network => "network",
            SandboxCapability::FilesystemRead => "fs_read",
            SandboxCapability::FilesystemWrite => "fs_write",
            SandboxCapability::CompositorControl => "compositor",
            SandboxCapability::Input => "input",
            SandboxCapability::Rpc => "rpc",
            SandboxCapability::None => "none",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SandboxPolicy {
    pub allow: Vec<SandboxCapability>,
}

impl Default for SandboxPolicy {
    fn default() -> Self {
        SandboxPolicy { allow: Vec::new() }
    }
}

impl SandboxPolicy {
    pub fn from_strings(capabilities: &[String]) -> Self {
        SandboxPolicy {
            allow: capabilities
                .iter()
                .map(|c| SandboxCapability::from_str(c))
                .collect(),
        }
    }

    pub fn permits(&self, capability: SandboxCapability) -> bool {
        self.allow.contains(&capability)
    }
}

pub struct PluginSandbox {
    pub policy: SandboxPolicy,
    pub hostname: String,
    pub time_limit_ms: u64,
}

impl Default for PluginSandbox {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginSandbox {
    pub fn new() -> Self {
        PluginSandbox {
            policy: SandboxPolicy::default(),
            hostname: "waydri-sandbox".to_string(),
            time_limit_ms: 0,
        }
    }

    pub fn with_policy(mut self, policy: SandboxPolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn check(&self, capability: SandboxCapability) -> Result<(), String> {
        if self.policy.permits(capability) {
            Ok(())
        } else {
            Err(format!(
                "plugin requires capability {} that the sandbox denies",
                capability.as_str()
            ))
        }
    }
}