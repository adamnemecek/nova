pub use specs::storage::{
  BTreeStorage, DenseVecStorage, FlaggedStorage, HashMapStorage, NullStorage, ReadStorage,
  VecStorage, WriteStorage,
};

pub type FlaggedBTreeStorage<T> = FlaggedStorage<T, VecStorage<T>>;
