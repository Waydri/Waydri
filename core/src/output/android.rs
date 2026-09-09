use super::{BackendKind, Output, OutputMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidConnector {
    PrimaryDisplay,
    ExternalDisplay,
    VirtualDisplay,
}

impl AndroidConnector {
    pub fn name(&self) -> &'static str {
        match self {
            AndroidConnector::PrimaryDisplay => "primary",
            AndroidConnector::ExternalDisplay => "external",
            AndroidConnector::VirtualDisplay => "virtual",
        }
    }

    pub fn backend_kind(&self) -> BackendKind {
        BackendKind::Android
    }

    pub fn default_mode(&self) -> OutputMode {
        match self {
            AndroidConnector::PrimaryDisplay => OutputMode::new(1920, 1080, 120),
            AndroidConnector::ExternalDisplay => OutputMode::new(3840, 2160, 60),
            AndroidConnector::VirtualDisplay => OutputMode::new(1280, 720, 60),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AndroidOutputConfig {
    pub connector: AndroidConnector,
    pub surface_width: u32,
    pub surface_height: u32,
}

impl AndroidOutputConfig {
    pub fn new(connector: AndroidConnector, width: u32, height: u32) -> Self {
        AndroidOutputConfig {
            connector,
            surface_width: width,
            surface_height: height,
        }
    }

    pub fn to_output(&self, id: u64) -> Output {
        let backend = self.connector.backend_kind();
        let mut output = Output::headless(
            id,
            format!("android-{}", self.connector.name()),
            self.surface_width,
            self.surface_height,
            self.connector.default_mode().refresh_hz(),
        );
        output.backend = backend;
        output
    }
}

pub fn displays_available() -> Vec<AndroidConnector> {
    vec![
        AndroidConnector::PrimaryDisplay,
        AndroidConnector::ExternalDisplay,
    ]
}