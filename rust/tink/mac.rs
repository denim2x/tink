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
use util::Result;

/**
  Interface for Message Authentication Codes (MAC).
 
  <h3>Security guarantees</h3>
 
  <p>Message Authentication Codes provide symmetric message authentication. Instances implementing
  this interface are secure against existential forgery under chosen plaintext attack, and can be
  deterministic or randomized. This interface should be used for authentication only, and not for
  other purposes like generation of pseudorandom bytes.
 */
pub trait Mac {
  /**
    Computes message authentication code (MAC) for {@code data}.
   
    @return MAC value
   */
  fn compute_mac(&self, data: &[u8]) -> Result<&[u8]>;
  
  /**
    Verifies whether {@code mac} is a correct authentication code (MAC) for {@code data}.
   
    @throws GeneralSecurityError if {@code mac} is not a correct MAC for {@code data}
   */
  fn verify_mac(&self, mac: &[u8], data: &[u8]) -> Result<()>;
}
