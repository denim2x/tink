// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
////////////////////////////////////////////////////////////////////////////////

use std::mem;

type String = &'static str;

pub enum Result<T> {
  Ok(T),
  Error(String),
  GeneralSecurityError(String),
  IOError(String),
  BufferUnderflowError(String),
}

impl Write {
  fn write(&mut self, val: u32) -> Result<usize> {
    self.write(transmute<u32, &[u8]>(val));
  }
}
