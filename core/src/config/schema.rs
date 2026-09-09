#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    String,
    Integer,
    Float,
    Boolean,
    StringList,
    Color,
    Enum,
}

#[derive(Debug, Clone)]
pub struct SchemaField {
    pub name: &'static str,
    pub value_type: ValueType,
    pub required: bool,
    pub default: Option<&'static str>,
    pub allowed: Option<&'static [&'static str]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaSection {
    General,
    Output,
    Input,
    Layout,
    Animation,
    Effects,
    Theme,
    Ipc,
}

pub struct ConfigSchema {
    pub fields: &'static [SchemaField],
}

impl ConfigSchema {
    pub const FIELDS: &'static [SchemaField] = &[
        SchemaField {
            name: "name",
            value_type: ValueType::String,
            required: false,
            default: Some("Waydri"),
            allowed: None,
        },
        SchemaField {
            name: "max_fps",
            value_type: ValueType::Integer,
            required: false,
            default: Some("120"),
            allowed: None,
        },
        SchemaField {
            name: "border_size",
            value_type: ValueType::Integer,
            required: false,
            default: Some("2"),
            allowed: None,
        },
        SchemaField {
            name: "gaps",
            value_type: ValueType::Integer,
            required: false,
            default: Some("8"),
            allowed: None,
        },
        SchemaField {
            name: "default_layout",
            value_type: ValueType::Enum,
            required: false,
            default: Some("master_stack"),
            allowed: Some(&[
                "master_stack",
                "dwindle",
                "grid",
                "custom",
                "dynamic",
            ]),
        },
        SchemaField {
            name: "log_level",
            value_type: ValueType::Enum,
            required: false,
            default: Some("info"),
            allowed: Some(&["trace", "debug", "info", "warn", "error"]),
        },
        SchemaField {
            name: "blur_enabled",
            value_type: ValueType::Boolean,
            required: false,
            default: Some("true"),
            allowed: None,
        },
        SchemaField {
            name: "blur_radius",
            value_type: ValueType::Float,
            required: false,
            default: Some("8.0"),
            allowed: None,
        },
        SchemaField {
            name: "shadows_enabled",
            value_type: ValueType::Boolean,
            required: false,
            default: Some("true"),
            allowed: None,
        },
        SchemaField {
            name: "shadow_opacity",
            value_type: ValueType::Float,
            required: false,
            default: Some("0.6"),
            allowed: None,
        },
    ];

    pub fn validate(section: SchemaSection, key: &str, value: &str) -> Result<(), String> {
        let _ = section;
        for field in Self::FIELDS {
            if field.name == key {
                match field.value_type {
                    ValueType::Boolean => {
                        if value != "true" && value != "false" {
                            return Err(format!("field {key} must be a boolean"));
                        }
                    }
                    ValueType::Integer => {
                        if value.parse::<i64>().is_err() {
                            return Err(format!("field {key} must be an integer"));
                        }
                    }
                    ValueType::Float => {
                        if value.parse::<f64>().is_err() {
                            return Err(format!("field {key} must be a float"));
                        }
                    }
                    ValueType::Enum => {
                        if let Some(allowed) = &field.allowed {
                            if !allowed.contains(&value) {
                                return Err(format!(
                                    "field {key} must be one of: {}",
                                    allowed.join(", ")
                                ));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}