//! A Rust structure that implements a serializer/deserializer corresponding to `hkColor`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 1
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkColor<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkColor"`: Name of this class.
    #[serde(default = "HkColor::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x106b96ce`: Unique value of this class.
    #[serde(default = "HkColor::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkColorHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkColorHkParam<'a>>
}

impl HkColor<'_> {
    /// Return `"hkColor"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkColor".into()
    }

    /// Return `"0x106b96ce"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x106b96ce".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkColorHkParam<'a> {
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkColorHkParam<'de>, "@name",
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ExtendedColors {
    #[serde(rename = "MAROON")]
    Maroon = -8388608,
    #[serde(rename = "DARKRED")]
    Darkred = -7667712,
    #[serde(rename = "RED")]
    Red = -65536,
    #[serde(rename = "LIGHTPINK")]
    Lightpink = -18751,
    #[serde(rename = "CRIMSON")]
    Crimson = -2354116,
    #[serde(rename = "PALEVIOLETRED")]
    Palevioletred = -2396013,
    #[serde(rename = "HOTPINK")]
    Hotpink = -38476,
    #[serde(rename = "DEEPPINK")]
    Deeppink = -60269,
    #[serde(rename = "MEDIUMVIOLETRED")]
    Mediumvioletred = -3730043,
    #[serde(rename = "PURPLE")]
    Purple = -8388480,
    #[serde(rename = "DARKMAGENTA")]
    Darkmagenta = -7667573,
    #[serde(rename = "ORCHID")]
    Orchid = -2461482,
    #[serde(rename = "THISTLE")]
    Thistle = -2572328,
    #[serde(rename = "PLUM")]
    Plum = -2252579,
    #[serde(rename = "VIOLET")]
    Violet = -1146130,
    #[serde(rename = "FUCHSIA")]
    Fuchsia = -65281,
    #[serde(rename = "MAGENTA")]
    Magenta = -65281,
    #[serde(rename = "MEDIUMORCHID")]
    Mediumorchid = -4565549,
    #[serde(rename = "DARKVIOLET")]
    Darkviolet = -7077677,
    #[serde(rename = "DARKORCHID")]
    Darkorchid = -6737204,
    #[serde(rename = "BLUEVIOLET")]
    Blueviolet = -7722014,
    #[serde(rename = "INDIGO")]
    Indigo = -11861886,
    #[serde(rename = "MEDIUMPURPLE")]
    Mediumpurple = -7114533,
    #[serde(rename = "SLATEBLUE")]
    Slateblue = -9807155,
    #[serde(rename = "MEDIUMSLATEBLUE")]
    Mediumslateblue = -8689426,
    #[serde(rename = "DARKBLUE")]
    Darkblue = -16777077,
    #[serde(rename = "MEDIUMBLUE")]
    Mediumblue = -16777011,
    #[serde(rename = "BLUE")]
    Blue = -16776961,
    #[serde(rename = "NAVY")]
    Navy = -16777088,
    #[serde(rename = "MIDNIGHTBLUE")]
    Midnightblue = -15132304,
    #[serde(rename = "DARKSLATEBLUE")]
    Darkslateblue = -12042869,
    #[serde(rename = "ROYALBLUE")]
    Royalblue = -12490271,
    #[serde(rename = "CORNFLOWERBLUE")]
    Cornflowerblue = -10185235,
    #[serde(rename = "LIGHTSTEELBLUE")]
    Lightsteelblue = -5192482,
    #[serde(rename = "ALICEBLUE")]
    Aliceblue = -984833,
    #[serde(rename = "GHOSTWHITE")]
    Ghostwhite = -460545,
    #[serde(rename = "LAVENDER")]
    Lavender = -1644806,
    #[serde(rename = "DODGERBLUE")]
    Dodgerblue = -14774017,
    #[serde(rename = "STEELBLUE")]
    Steelblue = -12156236,
    #[serde(rename = "DEEPSKYBLUE")]
    Deepskyblue = -16728065,
    #[serde(rename = "SLATEGRAY")]
    Slategray = -9404272,
    #[serde(rename = "LIGHTSLATEGRAY")]
    Lightslategray = -8943463,
    #[serde(rename = "LIGHTSKYBLUE")]
    Lightskyblue = -7876870,
    #[serde(rename = "SKYBLUE")]
    Skyblue = -7876885,
    #[serde(rename = "LIGHTBLUE")]
    Lightblue = -5383962,
    #[serde(rename = "TEAL")]
    Teal = -16744320,
    #[serde(rename = "DARKCYAN")]
    Darkcyan = -16741493,
    #[serde(rename = "DARKTURQUOISE")]
    Darkturquoise = -16724271,
    #[serde(rename = "CYAN")]
    Cyan = -16711681,
    #[serde(rename = "MEDIUMTURQUOISE")]
    Mediumturquoise = -12004916,
    #[serde(rename = "CADETBLUE")]
    Cadetblue = -10510688,
    #[serde(rename = "PALETURQUOISE")]
    Paleturquoise = -5247250,
    #[serde(rename = "LIGHTCYAN")]
    Lightcyan = -2031617,
    #[serde(rename = "AZURE")]
    Azure = -983041,
    #[serde(rename = "LIGHTSEAGREEN")]
    Lightseagreen = -14634326,
    #[serde(rename = "TURQUOISE")]
    Turquoise = -12525360,
    #[serde(rename = "POWDERBLUE")]
    Powderblue = -5185306,
    #[serde(rename = "DARKSLATEGRAY")]
    Darkslategray = -13676721,
    #[serde(rename = "AQUAMARINE")]
    Aquamarine = -8388652,
    #[serde(rename = "MEDIUMSPRINGGREEN")]
    Mediumspringgreen = -16713062,
    #[serde(rename = "MEDIUMAQUAMARINE")]
    Mediumaquamarine = -10039894,
    #[serde(rename = "SPRINGGREEN")]
    Springgreen = -16711809,
    #[serde(rename = "MEDIUMSEAGREEN")]
    Mediumseagreen = -12799119,
    #[serde(rename = "SEAGREEN")]
    Seagreen = -13726889,
    #[serde(rename = "LIMEGREEN")]
    Limegreen = -13447886,
    #[serde(rename = "DARKGREEN")]
    Darkgreen = -16751616,
    #[serde(rename = "GREEN")]
    Green = -16744448,
    #[serde(rename = "LIME")]
    Lime = -16711936,
    #[serde(rename = "FORESTGREEN")]
    Forestgreen = -14513374,
    #[serde(rename = "DARKSEAGREEN")]
    Darkseagreen = -7357297,
    #[serde(rename = "LIGHTGREEN")]
    Lightgreen = -7278960,
    #[serde(rename = "PALEGREEN")]
    Palegreen = -6751336,
    #[serde(rename = "MINTCREAM")]
    Mintcream = -655366,
    #[serde(rename = "HONEYDEW")]
    Honeydew = -983056,
    #[serde(rename = "CHARTREUSE")]
    Chartreuse = -8388864,
    #[serde(rename = "LAWNGREEN")]
    Lawngreen = -8586240,
    #[serde(rename = "OLIVEDRAB")]
    Olivedrab = -9728477,
    #[serde(rename = "DARKOLIVEGREEN")]
    Darkolivegreen = -11179217,
    #[serde(rename = "YELLOWGREEN")]
    Yellowgreen = -6632142,
    #[serde(rename = "GREENYELLOW")]
    Greenyellow = -5374161,
    #[serde(rename = "BEIGE")]
    Beige = -657956,
    #[serde(rename = "LINEN")]
    Linen = -331546,
    #[serde(rename = "LIGHTGOLDENRODYELLOW")]
    Lightgoldenrodyellow = -329006,
    #[serde(rename = "OLIVE")]
    Olive = -8355840,
    #[serde(rename = "YELLOW")]
    Yellow = -256,
    #[serde(rename = "LIGHTYELLOW")]
    Lightyellow = -32,
    #[serde(rename = "IVORY")]
    Ivory = -16,
    #[serde(rename = "DARKKHAKI")]
    Darkkhaki = -4343957,
    #[serde(rename = "KHAKI")]
    Khaki = -989556,
    #[serde(rename = "PALEGOLDENROD")]
    Palegoldenrod = -1120086,
    #[serde(rename = "WHEAT")]
    Wheat = -663885,
    #[serde(rename = "GOLD")]
    Gold = -10496,
    #[serde(rename = "LEMONCHIFFON")]
    Lemonchiffon = -1331,
    #[serde(rename = "PAPAYAWHIP")]
    Papayawhip = -4139,
    #[serde(rename = "DARKGOLDENROD")]
    Darkgoldenrod = -4684277,
    #[serde(rename = "GOLDENROD")]
    Goldenrod = -2448096,
    #[serde(rename = "ANTIQUEWHITE")]
    Antiquewhite = -332841,
    #[serde(rename = "CORNSILK")]
    Cornsilk = -1828,
    #[serde(rename = "OLDLACE")]
    Oldlace = -133658,
    #[serde(rename = "MOCCASIN")]
    Moccasin = -6987,
    #[serde(rename = "NAVAJOWHITE")]
    Navajowhite = -8531,
    #[serde(rename = "ORANGE")]
    Orange = -23296,
    #[serde(rename = "BISQUE")]
    Bisque = -6972,
    #[serde(rename = "TAN")]
    Tan = -2968436,
    #[serde(rename = "DARKORANGE")]
    Darkorange = -29696,
    #[serde(rename = "BURLYWOOD")]
    Burlywood = -2180985,
    #[serde(rename = "SADDLEBROWN")]
    Saddlebrown = -7650029,
    #[serde(rename = "SANDYBROWN")]
    Sandybrown = -744352,
    #[serde(rename = "BLANCHEDALMOND")]
    Blanchedalmond = -5171,
    #[serde(rename = "LAVENDERBLUSH")]
    Lavenderblush = -3851,
    #[serde(rename = "SEASHELL")]
    Seashell = -2578,
    #[serde(rename = "FLORALWHITE")]
    Floralwhite = -1296,
    #[serde(rename = "SNOW")]
    Snow = -1286,
    #[serde(rename = "PERU")]
    Peru = -3308225,
    #[serde(rename = "PEACHPUFF")]
    Peachpuff = -9543,
    #[serde(rename = "CHOCOLATE")]
    Chocolate = -2987746,
    #[serde(rename = "SIENNA")]
    Sienna = -6270419,
    #[serde(rename = "LIGHTSALMON")]
    Lightsalmon = -24454,
    #[serde(rename = "CORAL")]
    Coral = -32944,
    #[serde(rename = "DARKSALMON")]
    Darksalmon = -1468806,
    #[serde(rename = "MISTYROSE")]
    Mistyrose = -6943,
    #[serde(rename = "ORANGERED")]
    Orangered = -47872,
    #[serde(rename = "SALMON")]
    Salmon = -360334,
    #[serde(rename = "TOMATO")]
    Tomato = -40121,
    #[serde(rename = "ROSYBROWN")]
    Rosybrown = -4419697,
    #[serde(rename = "PINK")]
    Pink = -16181,
    #[serde(rename = "INDIANRED")]
    Indianred = -3318692,
    #[serde(rename = "LIGHTCORAL")]
    Lightcoral = -1015680,
    #[serde(rename = "BROWN")]
    Brown = -5952982,
    #[serde(rename = "FIREBRICK")]
    Firebrick = -5103070,
    #[serde(rename = "BLACK")]
    Black = -16777216,
    #[serde(rename = "DIMGRAY")]
    Dimgray = -9868951,
    #[serde(rename = "GRAY")]
    Gray = -8355712,
    #[serde(rename = "DARKGRAY")]
    Darkgray = -5658199,
    #[serde(rename = "SILVER")]
    Silver = -4144960,
    #[serde(rename = "LIGHTGREY")]
    Lightgrey = -2894893,
    #[serde(rename = "GAINSBORO")]
    Gainsboro = -2302756,
    #[serde(rename = "WHITESMOKE")]
    Whitesmoke = -657931,
    #[serde(rename = "WHITE")]
    White = -1,
    #[serde(rename = "GREY")]
    Grey = -7829368,
    #[serde(rename = "GREY25")]
    Grey25 = -12566464,
    #[serde(rename = "GREY50")]
    Grey50 = -8355712,
    #[serde(rename = "GREY75")]
    Grey75 = -4144960,
}
