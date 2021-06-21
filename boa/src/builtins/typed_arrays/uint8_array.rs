use crate::builtins::typed_arrays::typed_array::TypedArrayInstance;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Uint8Array;

impl TypedArrayInstance for Uint8Array {
    const BYTES_PER_ELEMENT: usize = 1;
    const NAME: &'static str = "Uint8Array";
}
