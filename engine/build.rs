use std::error::Error;

fn main() -> Result<(), Box<Error>> {
  use glsl_to_spirv::ShaderType;

  // Tell the build script to only run again if the source shaders change.
  println!("cargo:rerun-if-changed=src/graphics/rendering/shaders/hlsl");

  // Create destination path if necessary
  std::fs::create_dir_all("src/graphics/rendering/shaders/spirv")?;

  // Build each shader in the path.
  for entry in std::fs::read_dir("src/graphics/rendering/shaders/hlsl")? {
    let entry = entry?;

    if entry.file_type()?.is_file() {
      let in_path = entry.path();

      // Support only vertex and fragment shaders currently
      let shader_type = in_path
        .extension()
        .and_then(|ext| match ext.to_string_lossy().as_ref() {
          "vert" => Some(ShaderType::Vertex),
          "frag" => Some(ShaderType::Fragment),
          _ => None,
        });

      if let Some(shader_type) = shader_type {
        use std::io::Read;

        let source = std::fs::read_to_string(&in_path)?;
        let mut compiled_file = glsl_to_spirv::compile(&source, shader_type)?;

        let mut compiled_bytes = Vec::new();
        compiled_file.read_to_end(&mut compiled_bytes)?;

        let out_path = format!(
          "src/graphics/rendering/shaders/spirv/{}.spv",
          in_path.file_name().unwrap().to_string_lossy()
        );

        std::fs::write(&out_path, &compiled_bytes)?;

        println!("cargo:rerun-if-changed={}", entry.path().to_str().unwrap());
      }
    }
  }

  Ok(())
}
