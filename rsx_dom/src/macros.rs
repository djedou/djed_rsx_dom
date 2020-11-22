
#[macro_export]
macro_rules! fragment {
    (@ $parent:ident; DOMNode::from
        ((
            $name:expr,
            vec![
                $( $attributes:tt )*
            ],
            vec![
                $( $children:tt )*
            ]
        ))
        $( $tail:tt )*
    ) => {
        {
            let mut parent = $parent.append(DOMNode::from(($name, vec![$( $attributes )*])));
            fragment! { @ parent; $( $children )* }
        }
        fragment! { @ $parent; $( $tail )* }
    };

    (@ $parent:ident; DOMNode::from
        ((
            $name:expr,
            DOMChildren::from(vec![
                $( $children:tt )*
            ])
        ))
        $( $tail:tt )*
    ) => {
        {
            let mut parent = $parent.append(DOMNode::from($name));
            fragment! { @ parent; $( $children )* }
        }
        fragment! { @ $parent; $( $tail )* }
    };

    (@ $parent:ident; DOMNode::from
        ((
            $name:expr,
            DOMAttributes:from(vec![
                $( $attributes:tt )*
            ])
        ))
        $( $tail:tt )*
    ) => {
        $parent.append(DOMNode::from(($name, vec![$( $attributes )*])));
        fragment! { @ $parent; $( $tail )* }
    };

    (@ $parent:ident; DOMNode::from
        (
            $code:block
        )
        $( $tail:tt )*
    ) => {
        {
            let node = DOMNode::from($code);
            if node.is_shadow_host() {
                $parent.append_tree(node.shadow_dom());
            } else if !node.is_void() {
                $parent.append(node);
            }
        }
        fragment! { @ $parent; $( $tail )* }
    };

    (@ $parent:ident; DOMNode::from
        (
            $name:expr
        )
        $( $tail:tt )*
    ) => {
        $parent.append(DOMNode::from($name));
        fragment! { @ $parent; $( $tail )* }
    };

    (@ $parent:ident; , $( $tail:tt )* ) => {
        fragment! { @ $parent; $( $tail )* }
    };

    (@ $parent:ident; ) => {};

    ($( $tt:tt )*) => {{
        let mut fragment = DOMTree::default();
        {
            let mut parent = fragment.root_mut();
            fragment! { @ parent; $( $tt )* }
        }
        fragment
    }};
}

macro_rules! impl_text_node_from_stringifiable {
    ($src: ty) => {
        impl<E, S, C, L> From<$src> for DOMNode<E, S, C, L>
        where
            E: TGenericEvent,
            S: TStyleDeclarations,
            C: TComputedStyles<Styles = S>,
            L: TLayoutNode<Styles = S>
        {
            fn from(value: $src) -> Self {
                DOMNode::new(DOMData::Text(DOMTextNode {
                    content: DOMText::from(value.to_string())
                }))
            }
        }
    };
}

macro_rules! impl_number_attribute_from_countable {
    ($src: ty) => {
        impl<E, S, C, L> From<$src> for DOMAttributeValue<E, S, C, L> {
            
            fn from(value: $src) -> Self {
                DOMAttributeValue::Number(value as f64)
            }
        }
    };
}
