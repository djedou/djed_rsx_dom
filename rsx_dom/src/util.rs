
use crate::types::{Closure, DOMAttribute, DOMAttributeName, DOMAttributeValue, DOMText, EventType, KnownAttributeName};

pub fn is_event_listener<E, S, C, L>(attribute: &DOMAttribute<E, S, C, L>) -> Option<&Closure<E>> {
    use self::DOMAttributeName::KnownName;
    use self::DOMAttributeValue::EventListener;
    use self::KnownAttributeName::Style;
    match attribute {
        &DOMAttribute(KnownName(Style), EventListener(ref l)) => Some(l),
        _ => None
    }
}

pub fn is_src<E, S, C, L>(attribute: &DOMAttribute<E, S, C, L>) -> Option<&DOMText> {
    use self::DOMAttributeName::KnownName;
    use self::DOMAttributeValue::Str;
    use self::KnownAttributeName::Src;
    match attribute {
        &DOMAttribute(KnownName(Src), Str(ref s)) => Some(s),
        _ => None
    }
}

pub fn is_style<E, S, C, L>(attribute: &DOMAttribute<E, S, C, L>) -> Option<&S> {
    use self::DOMAttributeName::KnownName;
    use self::DOMAttributeValue::Styles;
    use self::KnownAttributeName::Style;
    match attribute {
        &DOMAttribute(KnownName(Style), Styles(ref s)) => Some(s),
        _ => None
    }
}

pub fn find_attribute<'a, E, S, C, L, I>(iter: I, name: &DOMAttributeName) -> Option<&'a DOMAttribute<E, S, C, L>>
where
    I: IntoIterator<Item = &'a DOMAttribute<E, S, C, L>>
{
    iter.into_iter().find(|v| &v.0 == name)
}

pub fn find_event_listener<'a, E: 'a, S: 'a, C: 'a, L: 'a, I>(iter: I, ty: EventType) -> Option<&'a Closure<E>>
where
    I: IntoIterator<Item = &'a DOMAttribute<E, S, C, L>>
{
    let name = DOMAttributeName::EventType(ty);
    is_event_listener(find_attribute(iter, &name)?)
}

pub fn find_src<'a, E: 'a, S: 'a, C: 'a, L: 'a, I>(iter: I) -> Option<&'a DOMText>
where
    I: IntoIterator<Item = &'a DOMAttribute<E, S, C, L>>
{
    let name = DOMAttributeName::KnownName(KnownAttributeName::Src);
    is_src(find_attribute(iter, &name)?)
}

pub fn find_styles<'a, E: 'a, S: 'a, C: 'a, L: 'a, I>(iter: I) -> Option<&'a S>
where
    I: IntoIterator<Item = &'a DOMAttribute<E, S, C, L>>
{
    let name = DOMAttributeName::KnownName(KnownAttributeName::Style);
    is_style(find_attribute(iter, &name)?)
}
