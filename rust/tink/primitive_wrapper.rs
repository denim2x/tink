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

use std::any::TypeId;

mod util;
use util::Result;

/**
  Basic interface for wrapping a primitive.
 
  <p>A PrimitiveSet can be wrapped by a single primitive in order to fulfil a cryptographic task.
  This is done by the PrimitiveWrapper. Whenever a new primitive type is added to Tink, the user
  should define a new PrimitiveWrapper and register it by calling
  {@code Registry::register_primitive_wrapper}.
 */
pub trait PrimitiveWrapper<P> {
  /**
    Wraps a {@code PrimitiveSet} and returns a single instance.
   
    This has to be implemented when a new primitive type is added. 
   */
  fn wrap(&self, set: PrimitiveSet<P>) -> Result<P>;
  
  /**
    Returns the primitive {$code TypeId} of the primitive managed. Used for internal management.
   */
  fn primitive_type_id(&self) -> TypeId {
    TypeId::of::<P>;
  }
}
