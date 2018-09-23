
pub mod engine;

use self::engine::Engine;

struct Storage<E: Engine> {
    engine: E
}

impl<E: Engine> Storage<E> {}
