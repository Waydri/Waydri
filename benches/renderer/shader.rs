use std::time::Instant;

use waydri_core::renderer::shader::{ShaderSourceSet, ShaderType};

fn main() {
    let vertex = "#version 300 es\nlayout(location=0) in vec2 aPos;\nuniform mat4 uProj;\nvoid main() { gl_Position = uProj * vec4(aPos, 0.0, 1.0); }".to_string();
    let fragment = "#version 300 es\nprecision mediump float;\nuniform vec4 uColor;\nout vec4 fragColor;\nvoid main() { fragColor = uColor; }".to_string();
    let set = ShaderSourceSet::new(vertex, fragment);

    let start = Instant::now();
    for _ in 0..100_000 {
        let valid = set.validate();
        assert!(valid.is_ok());
    }
    let elapsed = start.elapsed();
    println!("shader validate: {:.0}/s", 100_000.0 / elapsed.as_secs_f64());

    let start = Instant::now();
    let mut uniforms = Vec::new();
    for _ in 0..100_000 {
        uniforms = set.uniforms();
    }
    let elapsed = start.elapsed();
    println!("uniform scan: {:.0}/s", 100_000.0 / elapsed.as_secs_f64());
    assert!(uniforms.contains(&"uColor".to_string()));
    assert!(uniforms.contains(&"uProj".to_string()));

    assert_eq!(ShaderType::Vertex.glsl_stage(), "vertex");
    assert_eq!(ShaderType::Fragment.glsl_stage(), "fragment");
}