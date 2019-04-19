// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod code;
mod shader;

pub use self::code::ShaderCode;
pub use self::shader::Shader;
pub use glsl_to_spirv::ShaderType as ShaderKind;
