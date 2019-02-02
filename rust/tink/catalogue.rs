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
use util::*;

/**
  A catalogue of {@link KeyManager} objects.
 
  <p>It is basically a map from a (key type, primitive name)-tuple to {@link KeyManager}-objects,
  that determines the implementation that handles the keys of the given key type.
 
  <p>Tink includes default per-primitive catalogues, but it also supports custom catalogues to
  enable user-defined configuration of run-time environment via {@link Registry}.
 
  <p>The template parameter {@code P} denotes the primitive corresponding to the {@link KeyManager}
  handled by this catalogue.
 */
pub trait Catalogue<P> {
  /**
    @return a {@link KeyManager} for the given {@code type_url}, {@code primitive_name}, and version
    at least {@code min_version} (if it exists in the catalogue).
   */
  fn key_manager(&self, type_url: String, primitive_name: String, min_version: u32) -> Result<KeyManager<P>>;
  
  /// Returns a new primitive wrapper for this primitive.
  fn primitive_wrapper(&self) -> Result<PrimitiveWrapper<P>>;
}
