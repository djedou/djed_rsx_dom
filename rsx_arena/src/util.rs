
#[inline]
pub unsafe fn as_mut<'a, V>(value: Option<&mut V>) -> Option<&'a mut V> {
    value.map(|v| v as *mut V).map(|v| &mut *v)
}
