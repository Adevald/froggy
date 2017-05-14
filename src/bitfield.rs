use {Epoch, StorageId};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PointerData(u64);

const INDEX_BITS: u8 = 40;
const EPOCH_BITS: u8 = 16;
const STORAGE_ID_BITS: u8 = 8;
const INDEX_MASK: u64 = (1 << INDEX_BITS) - 1;
const EPOCH_OFFSET: u8 = INDEX_BITS;
const EPOCH_MASK: u64 = ((1 << EPOCH_BITS) - 1) << EPOCH_OFFSET;
const STORAGE_ID_OFFSET: u8 = EPOCH_OFFSET + EPOCH_BITS;
const STORAGE_ID_MASK: u64 = ((1 << STORAGE_ID_BITS) - 1) << STORAGE_ID_OFFSET;

impl PointerData {
    #[inline]
    pub fn new(index: usize, epoch: Epoch, storage: StorageId) -> Self {
        let mut p = PointerData(0);
        p.set_index(index);
        p.set_epoch(epoch);
        p.set_storage_id(storage);
        p
    }

    #[inline]
    pub fn get_index(&self) -> usize {
        (self.0 & INDEX_MASK) as usize
    }

    #[inline]
    pub fn get_epoch(&self) -> Epoch {
        ((self.0 & EPOCH_MASK) >> EPOCH_OFFSET) as Epoch
    }

    #[inline]
    pub fn get_storage_id(&self) -> StorageId {
        ((self.0 & STORAGE_ID_MASK) >> STORAGE_ID_OFFSET) as StorageId
    }

    #[inline]
    pub fn set_index(&mut self, value: usize) {
        debug_assert_eq!(value >> INDEX_BITS, 0);
        self.0 = (self.0 & !INDEX_MASK) + value as u64;
    }

    #[inline]
    pub fn set_epoch(&mut self, value: Epoch) {
        //debug_assert_eq!(value >> EPOCH_BITS, 0);
        self.0 = (self.0 & !EPOCH_MASK) + ((value as u64) << EPOCH_OFFSET);
    }

    #[inline]
    pub fn set_storage_id(&mut self, value: StorageId) {
        //debug_assert_eq!(value >> EPOCH_BITS, 0);
        self.0 = (self.0 & !STORAGE_ID_MASK) + ((value as u64) << STORAGE_ID_OFFSET);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn sizes() {
        assert_eq!(INDEX_BITS + EPOCH_BITS + STORAGE_ID_BITS, 64);
        assert!(size_of::<Epoch>() * 8 >= EPOCH_BITS as usize);
        assert!(size_of::<StorageId>() * 8 >= STORAGE_ID_BITS as usize);
    }

    #[test]
    fn rw_pointer_data() {
        let mut pd = PointerData::new(1, 2, 3);
        assert_eq!(pd.get_index(), 1);
        assert_eq!(pd.get_epoch(), 2);
        assert_eq!(pd.get_storage_id(), 3);
        pd.set_index(2);
        assert_eq!(pd.get_index(), 2);
        pd.set_epoch(4);
        assert_eq!(pd.get_epoch(), 4);
        pd.set_storage_id(6);
        assert_eq!(pd.get_storage_id(), 6);
    }
}
