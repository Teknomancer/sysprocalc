use crate::RegisterDescriptor;
use std::sync::LazyLock;

pub static EFER: LazyLock<RegisterDescriptor> = LazyLock::new(|| {
    toml::from_str(r#"
        arch       = "x86"
        device     = "cpu"
        name       = "efer"
        desc       = "Extended Feature Register"
        bit_count  = 32
        byte_order = "LittleEndian"
        bit_ranges = [
            { start=0,  end=0,  kind="Normal", show=true, name="SCE",   short="SC Extensions",       long="System Call Extensions" },
            { start=8,  end=8,  kind="Normal", show=true, name="LME",   short="LM Enable",           long="Long Mode Enable" },
            { start=10, end=10, kind="Normal", show=true, name="LMA",   short="LM Active",           long="Long Mode Active" },
            { start=11, end=11, kind="Normal", show=true, name="NXE",   short="NX Enable",           long="No-Execute Enable" },
            { start=12, end=12, kind="Normal", show=true, name="SVME",  short="SVM Enable",          long="Secure Virtual Machine Enable (AMD)" },
            { start=13, end=13, kind="Normal", show=true, name="LMSLE", short="LMSL Enable",         long="Long Mode Segment Limit Enable (AMD)" },
            { start=14, end=14, kind="Normal", show=true, name="FFXSR", short="Fast FXSAVE/FXRSTOR", long="Fast FXSAVE/FXRSTOR support" },
        ]
    "#).expect("Failed to parse TOML")
});

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::RangeInclusive;
    use crate::{BitRange, BitRangeKind, ByteOrder};

    #[test]
    fn test_x86_cpu_efer() {
        let bits = Vec::from( [
            BitRange::new(RangeInclusive::new(0, 0),
                          BitRangeKind::Normal,
                          true,
                          String::from("SCE"),
                          String::from("SC Extensions"),
                          String::from("System Call Extensions")),
            BitRange::new(RangeInclusive::new(8, 8),
                          BitRangeKind::Normal,
                          true,
                          String::from("LME"),
                          String::from("LM Enable"),
                          String::from("Long Mode Enable")),
            BitRange::new(RangeInclusive::new(10, 10),
                          BitRangeKind::Normal,
                          true,
                          String::from("LMA"),
                          String::from("LM Active"),
                          String::from("Long Mode Active")),
            BitRange::new(RangeInclusive::new(11, 11),
                          BitRangeKind::Normal,
                          true,
                          String::from("NXE"),
                          String::from("NX Enable"),
                          String::from("No-Execute Enable")),
            BitRange::new(RangeInclusive::new(12, 12),
                          BitRangeKind::Normal,
                          true,
                          String::from("SVME"),
                          String::from("SVM Enable"),
                          String::from("Secure Virtual Machine Enable (AMD)")),
            BitRange::new(RangeInclusive::new(13, 13),
                          BitRangeKind::Normal,
                          true,
                          String::from("LMSLE"),
                          String::from("LMSL Enable"),
                          String::from("Long Mode Segment Limit Enable (AMD)")),
            BitRange::new(RangeInclusive::new(14, 14),
                         BitRangeKind::Normal,
                         true,
                         String::from("FFXSR"),
                         String::from("Fast FXSAVE/FXRSTOR"),
                         String::from("Fast FXSAVE/FXRSTOR support")),
            ] );

        assert_eq!(EFER.arch(), "x86");
        assert_eq!(EFER.device(), "cpu");
        assert_eq!(EFER.name(), "efer");
        assert_eq!(EFER.description(), "Extended Feature Register");
        assert_eq!(EFER.bit_count(), 32);
        assert_eq!(EFER.byte_order(), ByteOrder::LittleEndian);
        assert_eq!(EFER.bit_ranges(), &bits);
    }
}
