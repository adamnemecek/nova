// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::shaders::ShaderKind;
use std::ops::Deref;

pub struct ShaderCode(Vec<u8>);

impl ShaderCode {
  pub fn compile(kind: ShaderKind, source: impl AsRef<str>) -> Result<Self, String> {
    use std::io::Read;

    let mut output = glsl_to_spirv::compile(source.as_ref(), kind.clone())?;

    let mut spirv = Vec::with_capacity(output.metadata().map(|m| m.len()).unwrap_or(8192) as usize);

    output
      .read_to_end(&mut spirv)
      .expect("Could not read compiled shader");

    Ok(ShaderCode(spirv))
  }
}

impl Deref for ShaderCode {
  type Target = [u8];

  fn deref(&self) -> &[u8] {
    &self.0
  }
}
