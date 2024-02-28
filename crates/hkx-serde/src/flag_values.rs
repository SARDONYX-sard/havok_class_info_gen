//! Flags for field alignment needs, skipping serialization, etc.(The original C++ code is u16 bit flags)
//! - ref: havok_2010_2_0/Source/Common/Base/Refection/hkClassMember.h#L112
use bitflags::bitflags;
use core::fmt;

bitflags! {
    /// Flags for field alignment needs, skipping serialization, etc.(The original C++ code is u16 bit flags)
    /// - ref: havok_2010_2_0/Source/Common/Base/Refection/hkClassMember.h#L112
    #[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
    pub struct FlagValues: usize {
        const FLAGS_NONE = 0;
        /// Member has forced 8 byte alignment. 1 << 7
        const ALIGN8 = 128;
        /// Member has forced 16 byte alignment. 1 << 8
        const ALIGN16 = 256;
        /// The members memory contents is not owned by this object 1 << 9
        const NOT_OWNED = 512;
        /// This member should not be written when serializing 1 << 10
        const SERIALIZE_IGNORED = 1024;
    }
}

impl TryFrom<usize> for FlagValues {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        FlagValues::from_bits(value).ok_or_else(|| format!("Set invalid value: {}", value))
    }
}

impl fmt::Debug for FlagValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("FlagValues");

        if self.contains(FlagValues::FLAGS_NONE) {
            debug_struct.field("FLAGS_NONE", &true);
        }
        if self.contains(FlagValues::ALIGN8) {
            debug_struct.field("ALIGN8", &true);
        }
        if self.contains(FlagValues::ALIGN16) {
            debug_struct.field("ALIGN16", &true);
        }
        if self.contains(FlagValues::NOT_OWNED) {
            debug_struct.field("NOT_OWNED", &true);
        }
        if self.contains(FlagValues::SERIALIZE_IGNORED) {
            debug_struct.field("SERIALIZE_IGNORED", &true);
        }

        debug_struct.finish()
    }
}

impl fmt::Display for FlagValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.bits())
    }
}

impl FlagValues {
    /// Use a string format that is easy for humans to read.
    ///
    /// Like this.
    /// `"FLAGS_NONE | SERIALIZE_IGNORED"`
    pub fn human_readable(&self) -> String {
        let mut flags = Vec::new();

        if self.contains(FlagValues::FLAGS_NONE) {
            flags.push("FLAGS_NONE");
        }
        if self.contains(FlagValues::ALIGN8) {
            flags.push("ALIGN8");
        }
        if self.contains(FlagValues::ALIGN16) {
            flags.push("ALIGN16");
        }
        if self.contains(FlagValues::NOT_OWNED) {
            flags.push("NOT_OWNED");
        }
        if self.contains(FlagValues::SERIALIZE_IGNORED) {
            flags.push("SERIALIZE_IGNORED");
        }

        flags.join(" | ")
    }
}
