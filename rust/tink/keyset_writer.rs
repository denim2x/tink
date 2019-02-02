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

mod util;
mod proto;

use util::Result;
use proto::{ EncryptedKeyset, Keyset };

/// A KeysetWriter knows how to write a {@link Keyset} or an {@link EncryptedKeyset} to some storage system.
pub trait KeysetWriter {
  /// Tries to write a {@link Keyset} to some storage system.
  fn write(&mut self, keyset: Keyset) -> Result<()>;
  
  /// Tries to write an {@link EncryptedKeyset} to some storage system.
  fn write(&mut self, keyset: EncryptedKeyset) -> Result<()>;
}
