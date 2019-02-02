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

/// A KeysetReader knows how to read a {@link Keyset} or an {@link EncryptedKeyset} from some source.
pub trait KeysetReader {
  /**
    Tries to read and return a cleartext {@link Keyset}.
   
    @return the Keyset
   */
  fn read(&self) -> Result<Keyset>;
  
  /**
    Tries to read and return an {@link EncryptedKeyset}.
   
    @return the EncryptedKeyset
   */
  fn read_encrypted(&self) -> Result<EncryptedKeyset>;
}
