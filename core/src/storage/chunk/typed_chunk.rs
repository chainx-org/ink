// Copyright 2018-2019 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::storage::{
    alloc::{
        Allocate,
        AllocateUsing,
    },
    chunk::{
        RawChunk,
        RawChunkCell,
    },
    Key,
    NonCloneMarker,
};

/// A chunk of typed cells.
///
/// Provides interpreted access with offset to the associated contract storage slot.
///
/// # Guarantees
///
/// - `Owned`
/// - `Typed`
///
/// Read more about kinds of guarantees and their effect [here](../index.html#guarantees).
#[derive(Debug, PartialEq, Eq)]
pub struct TypedChunk<T> {
    /// The underlying chunk of cells.
    chunk: RawChunk,
    /// Marker that prevents this type from being `Copy` or `Clone` by accident.
    non_clone: NonCloneMarker<T>,
}

/// A single cell within a chunk of typed cells.
#[derive(Debug, PartialEq, Eq)]
pub struct TypedChunkCell<'a, T> {
    /// The underlying cell within the chunk of cells.
    cell: RawChunkCell<'a>,
    /// Marker that prevents this type from being `Copy` or `Clone` by accident.
    non_clone: NonCloneMarker<T>,
}

impl<T> AllocateUsing for TypedChunk<T> {
    unsafe fn allocate_using<A>(alloc: &mut A) -> Self
    where
        A: Allocate,
    {
        Self {
            chunk: RawChunk::allocate_using(alloc),
            non_clone: Default::default(),
        }
    }
}

impl<'a, T> TypedChunkCell<'a, T> {
    /// Creates a new raw chunk cell from the given key.
    ///
    /// # Safety
    ///
    /// This is unsafe since it doesn't check aliasing of cells.
    pub(self) unsafe fn new_unchecked(cell: RawChunkCell<'a>) -> Self {
        Self {
            cell,
            non_clone: NonCloneMarker::default(),
        }
    }

    /// Removes the value stored in this cell.
    pub fn clear(&mut self) {
        self.cell.clear()
    }
}

impl<'a, T> TypedChunkCell<'a, T>
where
    T: scale::Encode,
{
    /// Stores the value into the cell.
    pub fn store(&mut self, val: &T) {
        self.cell.store(&T::encode(val))
    }
}

impl<T> scale::Encode for TypedChunk<T> {
    fn encode_to<W: scale::Output>(&self, dest: &mut W) {
        self.chunk.encode_to(dest)
    }
}

impl<T> scale::Decode for TypedChunk<T> {
    #[cfg(not(feature = "old-codec"))]
    fn decode<I: scale::Input>(input: &mut I) -> Result<Self, scale::Error> {
        RawChunk::decode(input).map(|raw_chunk| {
            Self {
                chunk: raw_chunk,
                non_clone: NonCloneMarker::default(),
            }
        })
    }
    #[cfg(feature = "old-codec")]
    fn decode<I: scale::Input>(input: &mut I) -> Option<Self> {
        RawChunk::decode(input).map(|raw_chunk| {
            Self {
                chunk: raw_chunk,
                non_clone: NonCloneMarker::default(),
            }
        })
    }
}

impl<T> TypedChunk<T> {
    /// Returns the underlying key to the cells.
    ///
    /// # Note
    ///
    /// This is a low-level utility getter and should
    /// normally not be required by users.
    pub fn cells_key(&self) -> Key {
        self.chunk.cells_key()
    }

    /// Returns an accessor to the `n`-th cell.
    pub(crate) fn cell_at(&mut self, n: u32) -> TypedChunkCell<T> {
        unsafe { TypedChunkCell::new_unchecked(self.chunk.cell_at(n)) }
    }

    /// Removes the value stored in the `n`-th cell.
    pub fn clear(&mut self, n: u32) {
        self.cell_at(n).clear()
    }
}

impl<T> TypedChunk<T>
where
    T: scale::Decode,
{
    /// Loads the value stored in the `n`-th cell if any.
    ///
    /// # Panics
    ///
    /// If decoding of the loaded bytes fails.
    pub fn load(&self, n: u32) -> Option<T> {
        self.chunk.load(n).map(|loaded| {
            T::decode(&mut &loaded[..])
					// Maybe we should return an error instead of panicking.
					.expect(
						"[ink_core::TypedChunkCell::load] Error: \
						 failed upon decoding"
					)
        })
    }
}

impl<T> TypedChunk<T>
where
    T: scale::Encode,
{
    /// Stores the value into the `n`-th cell.
    pub fn store(&mut self, n: u32, val: &T) {
        self.cell_at(n).store(val)
    }
}

#[cfg(all(test, feature = "test-env"))]
mod tests {
    use super::*;

    use crate::{
        env,
        test_utils::run_test,
    };

    #[test]
    fn simple() {
        run_test(|| {
            const TEST_LEN: u32 = 5;

            let mut chunk = unsafe {
                let mut alloc =
                    crate::storage::alloc::BumpAlloc::from_raw_parts(Key([0x0; 32]));
                TypedChunk::allocate_using(&mut alloc)
            };

            // Invariants after initialization
            for i in 0..TEST_LEN {
                assert_eq!(chunk.load(i), None);
            }

            // Store some elements
            for i in 0..TEST_LEN {
                chunk.store(i, &i);
                assert_eq!(chunk.load(i), Some(i));
            }

            // Clear all elements.
            for i in 0..TEST_LEN {
                chunk.clear(i);
                assert_eq!(chunk.load(i), None);
            }
        })
    }

    #[test]
    fn count_reads_writes() {
        run_test(|| {
            const TEST_LEN: u32 = 5;

            let mut chunk = unsafe {
                let mut alloc =
                    crate::storage::alloc::BumpAlloc::from_raw_parts(Key([0x0; 32]));
                TypedChunk::allocate_using(&mut alloc)
            };

            // Reads and writes after init.
            assert_eq!(env::test::total_reads(), 0);
            assert_eq!(env::test::total_writes(), 0);

            // Loading from all cells.
            for i in 0..TEST_LEN {
                chunk.load(i);
                assert_eq!(env::test::total_reads(), i as u64 + 1);
                assert_eq!(env::test::total_writes(), 0);
            }
            assert_eq!(env::test::total_reads(), TEST_LEN as u64);
            assert_eq!(env::test::total_writes(), 0);

            // Writing to all cells.
            for i in 0..TEST_LEN {
                chunk.store(i, &i);
                assert_eq!(env::test::total_reads(), TEST_LEN as u64);
                assert_eq!(env::test::total_writes(), i as u64 + 1);
            }
            assert_eq!(env::test::total_reads(), TEST_LEN as u64);
            assert_eq!(env::test::total_writes(), TEST_LEN as u64);

            // Loading multiple times from a single cell.
            const LOAD_REPEATS: usize = 3;
            for n in 0..LOAD_REPEATS {
                chunk.load(0);
                assert_eq!(env::test::total_reads(), TEST_LEN as u64 + n as u64 + 1);
                assert_eq!(env::test::total_writes(), TEST_LEN as u64);
            }
            assert_eq!(
                env::test::total_reads(),
                TEST_LEN as u64 + LOAD_REPEATS as u64
            );
            assert_eq!(env::test::total_writes(), TEST_LEN as u64);

            // Storing multiple times to a single cell.
            const STORE_REPEATS: usize = 3;
            for n in 0..STORE_REPEATS {
                chunk.store(0, &10);
                assert_eq!(
                    env::test::total_reads(),
                    TEST_LEN as u64 + LOAD_REPEATS as u64
                );
                assert_eq!(env::test::total_writes(), TEST_LEN as u64 + n as u64 + 1);
            }
            assert_eq!(
                env::test::total_reads(),
                TEST_LEN as u64 + LOAD_REPEATS as u64
            );
            assert_eq!(
                env::test::total_writes(),
                TEST_LEN as u64 + STORE_REPEATS as u64
            );
        })
    }
}
