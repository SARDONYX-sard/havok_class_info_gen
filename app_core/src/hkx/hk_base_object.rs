use serde::{Deserialize, Serialize};

/// # Hkxcmd Info
/// ```txt
/// Signature:  e0708a00
/// VTable:  TRUE
/// Name:  hkBaseObject
/// Parent:  HK_NULL
/// Size:  4
/// #IFace:  1
///  000  HK_NULL
/// #Enums:  0
/// #Members:  0
/// Version:  0
/// ```
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkBaseObject")]
pub struct HkBaseObject {
    #[serde(borrow)]
    #[serde(default = "HkBaseObject::signature")]
    #[serde(rename = "@signature")]
    /// Fixed value unique to each class: `0xc713064e`
    pub signature: &'static str,
}

impl HkBaseObject {
    #[inline]
    pub const fn signature() -> &'static str {
        "0xe0708a00"
    }
}
