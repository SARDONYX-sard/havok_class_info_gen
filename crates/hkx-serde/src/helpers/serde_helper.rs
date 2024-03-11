///The MIT License (MIT)
///
/// Copyright (c) 2016 Johann Tuffe

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! deserialize_variant {
    // Produce struct enum variant
    ( $de:expr, $enum:tt, $variant:ident {
        $(
            $(#[$meta:meta])*
            $field:ident : $typ:ty
        ),* $(,)?
    } ) => ({
        let var = {
            // Create anonymous type
            #[derive(serde::Deserialize)]
            struct $variant {
                $(
                    $(#[$meta])*
                    $field: $typ,
                )*
            }
            <$variant>::deserialize($de)?
        };
        // Due to https://github.com/rust-lang/rust/issues/86935 we cannot use
        // <$enum> :: $variant
        use $enum :: *;
        $variant {
            $($field: var.$field,)*
        }
    });

    // Produce newtype enum variant
    ( $de:expr, $enum:tt, $variant:ident($typ:ty) ) => ({
        use serde::Deserialize;
        let var = <$typ>::deserialize($de)?;
        <$enum> :: $variant(var)
    });

    // Produce unit enum variant
    ( $de:expr, $enum:tt, $variant:ident ) => ({
        serde::de::IgnoredAny::deserialize($de)?;
        <$enum> :: $variant
    });
}

/// Derived version of [`quick_xml::impl_deserialize_for_internally_tagged_enum!`](https://docs.rs/quick-xml/latest/quick_xml/macro.impl_deserialize_for_internally_tagged_enum.html)
///
/// In my version, the restriction of "first attribute" is removed and the attribute at any position is obtained.
#[macro_export(local_inner_macros)]
macro_rules! impl_deserialize_for_internally_tagged_enum {
    (
        $enum:ty,
        $tag:literal,
        $(
            ($variant_tag:literal => $($variant:tt)+ )
        ),* $(,)?
    ) => {
        impl<'de> serde::de::Deserialize<'de> for $enum {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                use serde::de::{Error, MapAccess, Visitor};

                // The Visitor struct is normally used for state, but none is needed
                struct TheVisitor;
                // The main logic of the deserializing happens in the Visitor trait
                impl<'de> Visitor<'de> for TheVisitor {
                    // The type that is being deserialized
                    type Value = $enum;

                    // Try to give a better error message when this is used wrong
                    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        f.write_str("expecting map with tag in ")?;
                        f.write_str($tag)
                    }

                    // The xml data is provided as an opaque map,
                    // that map is parsed into the type
                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: MapAccess<'de>,
                    {
                        // Here the assumption is made that only one attribute
                        // exists and it's the discriminator (enum "tag").
                        let mut tag = None;
                        while let Some((attribute, value)) = map.next_entry::<String, String>()? {
                            if attribute == $tag {
                                tag = Some(value);
                                break;
                            }
                        };
                        let tag = tag.ok_or_else(|| A::Error::missing_field($tag))?;

                        let de = serde::de::value::MapAccessDeserializer::new(map);
                        match tag.as_ref() {
                            $(
                                $variant_tag => Ok(deserialize_variant!( de, $enum, $($variant)+ )),
                            )*
                            _ => Err(A::Error::unknown_field(&tag, &[$($variant_tag),+])),
                        }
                    }
                }
                // Tell the deserializer to deserialize the data as a map,
                // using the TheVisitor as the decoder
                deserializer.deserialize_map(TheVisitor)
            }
        }
    }
}
