use std::borrow::Cow;

use zerocopy::{LayoutVerified, AsBytes, FromBytes, Unaligned};
use crate::{BytesEncode, BytesDecode};

/// Describes a type that is totally borrowed and doesn't
/// depends on any [memory alignment].
///
/// If you need to store a type that does depend on memory alignment
/// and that can be big it is recommended to use the [`CowType`].
///
/// [memory alignment]: https://doc.rust-lang.org/std/mem/fn.align_of.html
/// [`CowType`]: crate::types::CowType
pub struct UnalignedSlice<T>(std::marker::PhantomData<T>);

impl<T> BytesEncode for UnalignedSlice<T> where T: AsBytes + Unaligned {
    type EItem = [T];

    fn bytes_encode(item: &Self::EItem) -> Option<Cow<[u8]>> {
        Some(Cow::Borrowed(<[T] as AsBytes>::as_bytes(item)))
    }
}

impl<'a, T: 'a> BytesDecode<'a> for UnalignedSlice<T> where T: FromBytes + Unaligned {
    type DItem = &'a [T];

    fn bytes_decode(bytes: &'a [u8]) -> Option<Self::DItem> {
        LayoutVerified::<_, [T]>::new_slice_unaligned(bytes).map(LayoutVerified::into_slice)
    }
}
