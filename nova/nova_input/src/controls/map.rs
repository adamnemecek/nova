// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::controls::ControlBinding;
use nova_core::collections::{HashMap, HashSet};
use nova_core::quick_error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fs::File;
use std::io::{self, Read as _};
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ControlMap {
  #[serde(flatten)]
  pub bindings: HashMap<String, ControlBindingSet>,
}

impl ControlMap {
  pub fn load_file(path: impl AsRef<Path>) -> Result<Self, DeserializeError> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    Self::load(&buffer)
  }

  pub fn load(bytes: &[u8]) -> Result<Self, DeserializeError> {
    let map = toml::from_slice(&bytes)?;

    Ok(map)
  }
}

#[derive(Debug, Default)]
pub struct ControlBindingSet {
  pub positive: HashSet<ControlBinding>,
  pub negative: HashSet<ControlBinding>,
}

impl Serialize for ControlBindingSet {
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    use serde::ser::{SerializeSeq, SerializeStruct};

    if self.negative.is_empty() {
      let mut s = serializer.serialize_seq(Some(self.positive.len()))?;

      for binding in &self.positive {
        s.serialize_element(binding)?;
      }

      s.end()
    } else {
      let mut s = serializer.serialize_struct("ControlBindingSet", 2)?;

      s.serialize_field("positive", &self.positive)?;
      s.serialize_field("negative", &self.negative)?;

      s.end()
    }
  }
}

impl<'de> Deserialize<'de> for ControlBindingSet {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_any(DeserializeVisitor)
  }
}

struct DeserializeVisitor;

impl<'de> serde::de::Visitor<'de> for DeserializeVisitor {
  type Value = ControlBindingSet;

  fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str("a control binding set")
  }

  fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
  where
    A: serde::de::MapAccess<'de>,
  {
    use serde::de::Error;

    let mut set = ControlBindingSet::default();

    while let Some((field, value)) = map.next_entry()? {
      match field {
        "positive" => {
          set.positive = value;
        }

        "negative" => {
          set.negative = value;
        }

        field => return Err(Error::unknown_field(field, &["positive", "negative"])),
      }
    }

    Ok(set)
  }

  fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
  where
    A: serde::de::SeqAccess<'de>,
  {
    let mut positive = HashSet::default();

    while let Some(binding) = seq.next_element()? {
      positive.insert(binding);
    }

    Ok(ControlBindingSet {
      positive,
      negative: Default::default(),
    })
  }
}

quick_error! {
  #[derive(Debug)]
  pub enum DeserializeError {
    Io(err: io::Error) {
      from()
      description("io error")
      display("io error: {}", err)
    }

    Format(err: toml::de::Error) {
      from()
      description("invalid format")
      display("invalid format: {}", err)
    }
  }
}
