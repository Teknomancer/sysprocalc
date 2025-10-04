use crate::RegisterDescriptor;
use std::sync::LazyLock;

pub static CR0: LazyLock<RegisterDescriptor> = LazyLock::new(|| {
    toml::from_str(r#"
        arch       = "x86"
        device     = "cpu"
        name       = "cr0"
        desc       = "Control Register 0"
        bit_count  = 32
        byte_order = "LittleEndian"
        bit_ranges = [
            { start=0,  end=0,  kind="Normal", show=true, name="PE",    short="Protected Mode Enable", long="Protected Mode Enable." },
            { start=1,  end=1,  kind="Normal", show=true, name="MP",    short="Monitor Co-Processor",  long="Controls interaction of WAIT/FWAIT instructions with CR0.TS." },
            { start=2,  end=2,  kind="Normal", show=true, name="EM",    short="Emulation",             long="If set, no x87 FPU emulation." },
            { start=3,  end=3,  kind="Normal", show=true, name="TS",    short="Task Switched",         long="If set, save FPU context on FPU instruction after a task switch." },
            { start=4,  end=4,  kind="Normal", show=true, name="ET",    short="Extension Type",        long="On 386, specified if math coprocessor was an 80287 or 80387." },
            { start=5,  end=5,  kind="Normal", show=true, name="NE",    short="Numeric Error",         long="If set, enables x87 FPU error reporting." },
            { start=16, end=16, kind="Normal", show=true, name="WP",    short="Write Protect",         long="Controls whether the CPU can write to pages marked read-only." },
            { start=18, end=18, kind="Normal", show=true, name="AM",    short="Alignment Mask",        long="If set, enables alignment check for CPL=3 when EFLAGS.AC is set." },
            { start=29, end=29, kind="Normal", show=true, name="NW",    short="Not Write-Through",     long="Controls write-back or write-through for writes that hit the cache." },
            { start=30, end=30, kind="Normal", show=true, name="CD",    short="Cache Disable",         long="If set, disables memory caching." },
            { start=31, end=31, kind="Normal", show=true, name="PG",    short="Paging",                long="If set, enables memory paging." },
        ]
    "#).expect("Failed to parse TOML")
});

pub static EFER: LazyLock<RegisterDescriptor> = LazyLock::new(|| {
    toml::from_str(r#"
        arch       = "x86"
        device     = "cpu"
        name       = "efer"
        desc       = "Extended Feature Register"
        bit_count  = 32
        byte_order = "LittleEndian"
        bit_ranges = [
            { start=0,  end=0,  kind="Normal", show=true, name="SCE",   short="SC Extensions",       long="System Call Extensions." },
            { start=8,  end=8,  kind="Normal", show=true, name="LME",   short="LM Enable",           long="Long Mode Enable." },
            { start=10, end=10, kind="Normal", show=true, name="LMA",   short="LM Active",           long="Long Mode Active." },
            { start=11, end=11, kind="Normal", show=true, name="NXE",   short="NX Enable",           long="No-Execute Enable." },
            { start=12, end=12, kind="Normal", show=true, name="SVME",  short="SVM Enable",          long="Secure Virtual Machine Enable (AMD)." },
            { start=13, end=13, kind="Normal", show=true, name="LMSLE", short="LMSL Enable",         long="Long Mode Segment Limit Enable (AMD)." },
            { start=14, end=14, kind="Normal", show=true, name="FFXSR", short="Fast FXSAVE/FXRSTOR", long="Fast FXSAVE/FXRSTOR support." },
        ]
    "#).expect("Failed to parse TOML")
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BitRange, BitRangeKind, ByteOrder};
    use std::borrow::Cow;
    use std::ops::RangeInclusive;

    #[test]
    fn test_x86_cpu_efer() {
        let bits = Vec::from([
            BitRange::new(
                RangeInclusive::new(0, 0),
                BitRangeKind::Normal,
                true,
                Cow::Borrowed("SCE"),
                Cow::Borrowed("SC Extensions"),
                Cow::Borrowed("System Call Extensions."),
            ),
            BitRange::new(
                RangeInclusive::new(8, 8),
                BitRangeKind::Normal,
                true,
                Cow::Borrowed("LME"),
                Cow::Borrowed("LM Enable"),
                Cow::Borrowed("Long Mode Enable."),
            ),
            BitRange::new(
                RangeInclusive::new(10, 10),
                BitRangeKind::Normal,
                true,
                Cow::Borrowed("LMA"),
                Cow::Borrowed("LM Active"),
                Cow::Borrowed("Long Mode Active."),
            ),
            BitRange::new(
                RangeInclusive::new(11, 11),
                BitRangeKind::Normal,
                true,
                Cow::Borrowed("NXE"),
                Cow::Borrowed("NX Enable"),
                Cow::Borrowed("No-Execute Enable."),
            ),
            BitRange::new(
                RangeInclusive::new(12, 12),
                BitRangeKind::Normal,
                true,
                Cow::Borrowed("SVME"),
                Cow::Borrowed("SVM Enable"),
                Cow::Borrowed("Secure Virtual Machine Enable (AMD)."),
            ),
            BitRange::new(
                RangeInclusive::new(13, 13),
                BitRangeKind::Normal,
                true,
                Cow::Borrowed("LMSLE"),
                Cow::Borrowed("LMSL Enable"),
                Cow::Borrowed("Long Mode Segment Limit Enable (AMD)."),
            ),
            BitRange::new(
                RangeInclusive::new(14, 14),
                BitRangeKind::Normal,
                true,
                Cow::Borrowed("FFXSR"),
                Cow::Borrowed("Fast FXSAVE/FXRSTOR"),
                Cow::Borrowed("Fast FXSAVE/FXRSTOR support."),
            ),
        ]);

        assert_eq!(EFER.arch(), "x86");
        assert_eq!(EFER.device(), "cpu");
        assert_eq!(EFER.name(), "efer");
        assert_eq!(EFER.description(), "Extended Feature Register");
        assert_eq!(EFER.bit_count(), 32);
        assert_eq!(EFER.byte_order(), ByteOrder::LittleEndian);
        assert_eq!(EFER.bit_ranges(), &bits);
    }
}
