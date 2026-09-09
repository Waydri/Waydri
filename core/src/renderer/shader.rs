use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderType {
    Vertex,
    Fragment,
    Compute,
    Geometry,
}

impl ShaderType {
    pub fn glsl_stage(&self) -> &'static str {
        match self {
            ShaderType::Vertex => "vertex",
            ShaderType::Fragment => "fragment",
            ShaderType::Compute => "compute",
            ShaderType::Geometry => "geometry",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShaderSource {
    pub typ: ShaderType,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct ShaderSourceSet {
    pub vertex: ShaderSource,
    pub fragment: ShaderSource,
}

impl ShaderSourceSet {
    pub fn new(vertex: String, fragment: String) -> Self {
        ShaderSourceSet {
            vertex: ShaderSource {
                typ: ShaderType::Vertex,
                source: vertex,
            },
            fragment: ShaderSource {
                typ: ShaderType::Fragment,
                source: fragment,
            },
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        let (version, vs) = parse_version(&self.vertex.source);
        let (fversion, fs) = parse_version(&self.fragment.source);
        if version != fversion {
            return Err("vertex and fragment shaders declare different GLSL versions".to_string());
        }
        if !vs && !fs {
            return Err("missing #version directive".to_string());
        }
        validate_stage_bits(&self.vertex.source, &["gl_Position"])?;
        validate_stage_bits(&self.fragment.source, &["out ", "gl_FragColor"])?;
        Ok(())
    }

    pub fn uniforms(&self) -> Vec<String> {
        let mut found = HashSet::new();
        for line in self.vertex.source.lines().chain(self.fragment.source.lines()) {
            let line = line.trim();
            if line.starts_with("uniform ") {
                if let Some(mut name) = line.strip_prefix("uniform ") {
                    name = name.split_whitespace().last().unwrap_or("").trim_end_matches(';');
                    if let Some(short) = name.split(|c: char| !c.is_ascii_alphanumeric() && c != '_').next() {
                        if !short.is_empty() {
                            found.insert(short.to_string());
                        }
                    } else {
                        found.insert(name.to_string());
                    }
                }
            }
        }
        let mut list: Vec<String> = found.into_iter().collect();
        list.sort();
        list
    }
}

fn parse_version(source: &str) -> (u32, bool) {
    for line in source.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("#version") {
            let version: u32 = rest
                .split_whitespace()
                .next()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);
            return (version, true);
        }
    }
    (0, false)
}

fn validate_stage_bits(source: &str, required: &[&str]) -> Result<(), String> {
    for token in required {
        if !source.contains(token) {
            return Err(format!("shader missing required token `{token}`"));
        }
    }
    Ok(())
}