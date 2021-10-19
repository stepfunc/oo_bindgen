use std::ffi::CStr;

pub struct InnerByteIterator<'a> {
    item: crate::ffi::ByteValue,
    values: &'a [u8],
}

pub unsafe fn next_byte_value(it: *mut crate::InnerByteIterator) -> Option<&crate::ffi::ByteValue> {
    if it.is_null() {
        return None;
    }

    let it = &mut *it;

    match it.values {
        [] => None,
        [x, tail @ ..] => {
            it.item.value = *x;
            it.values = tail;
            Some(&it.item)
        }
    }
}

pub struct ChunkIterator<'a> {
    item: crate::ffi::Chunk<'a>,
    bytes: &'a [u8],
    chunk_size: usize,
}

pub unsafe fn next_chunk(it: *mut crate::ChunkIterator) -> Option<&crate::ffi::Chunk> {
    if it.is_null() {
        return None;
    }
    let it = &mut (*it);
    match it.bytes.get(..it.chunk_size) {
        None => None,
        Some(chunk) => {
            (*it.item.iter).values = chunk;
            it.bytes = match it.bytes.get(it.chunk_size..) {
                Some(x) => x,
                None => &[],
            };
            Some(&it.item)
        }
    }
}

pub unsafe fn iterate_string_by_chunks(
    value: &CStr,
    chunk_size: u32,
    callback: crate::ffi::ChunkReceiver,
) {
    let mut inner = InnerByteIterator {
        item: crate::ffi::ByteValue { value: 0 },
        values: &[],
    };
    let mut outer = ChunkIterator {
        item: crate::ffi::Chunk { iter: &mut inner },
        bytes: value.to_bytes(),
        chunk_size: chunk_size as usize,
    };

    callback.on_chunk(&mut outer)
}
