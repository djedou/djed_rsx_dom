
use std::convert::TryInto;

use djed_rsx_shared::traits::{
    event_traits::{TGenericEvent},
    layout_traits::{TLayoutNode},
    style_traits::{TComputedStyles, TStyleDeclarations}
};

use types::{DOMData, DOMTagName, KnownElementName};

impl<'a, E, S, C, L> TryInto<KnownElementName> for &'a DOMData<E, S, C, L>
where
    E: TGenericEvent,
    S: TStyleDeclarations,
    C: TComputedStyles,
    L: TLayoutNode
{
    type Error = ();

    fn try_into(self) -> Result<KnownElementName, Self::Error> {
        if let Some(&DOMTagName::KnownName(name)) = self.tag() {
            Ok(name)
        } else {
            Err(())
        }
    }
}
