
// note: https://www.khronos.org/opengl/wiki/Debug_Output
//   article about the debugging capabilities of opengl - integrate message severity with
//   logging levels and context type with cargo build type (find out if the cfg(debug) is possible)

use crate::context::targets::Target;

pub trait Name {
    type Name;

    // note: https://www.khronos.org/opengl/wiki/OpenGL_Object/Object_Name
    //  strings can be associated with system generated object names?

    fn name(&self) -> Self::Name;
}

// if Handle was to expect Target, then Target needs to know how to bind itself.
// Pros of separation:
//  Explicit typw
trait Resource {
    // do I really want to separate creation from actual resource acquisition?
    fn acquire(&mut self) {

    }

    fn release(self) {

    }
}

// Handle controls lifetime
struct Handle<R: Resource>(R);

impl<R> Handle<R> where R: Resource {

}

impl<R> Default for Handle<R> where R: Resource + Default {
    fn default() -> Self {
        Self(<R as Default>::default())
    }
}

impl<R> std::ops::Deref for Handle<R> where R: Resource {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R> Drop for Handle<R> where R: Resource {
    fn drop(&mut self) {
        self.0.release()
    }
}

struct Object<`Target: Target> {

}

struct PartialObject<T>