use crate::{BitRange, BitRangeKind, ByteOrder, RegisterDescriptor};
use std::ops::RangeInclusive;
use std::sync::LazyLock;
use serde::Deserialize;
use toml;

pub static X86_CPU_EFER: LazyLock<RegisterDescriptor> = LazyLock::new(|| {
toml::from_str(r#"
arch       = "X86"
device     = "CPU"
name       = "EFER"
desc       = "Extended Feature Register"
bit_count  = 64
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

    assert_eq!(X86_CPU_EFER.arch(), "X86");
    assert_eq!(X86_CPU_EFER.device(), "CPU");
    assert_eq!(X86_CPU_EFER.name(), "EFER");
    assert_eq!(X86_CPU_EFER.description(), "Extended Feature Register");
    assert_eq!(X86_CPU_EFER.bit_count(), 64);
    assert_eq!(X86_CPU_EFER.byte_order(), ByteOrder::LittleEndian);
    assert_eq!(X86_CPU_EFER.bit_ranges(), &bits);
}

/*

use crate::{BitSpan, BitSpanKind, BitGroup, ByteOrder};
use std::ops::RangeInclusive;

static ARCH: &str = "x86";
static DEVICE: &str = "cpu";

pub static CPU_X86_EFER_BITS: [BitSpan; 7] = [
    BitSpan {
        span:  RangeInclusive::new(0, 0),
        kind:  BitSpanKind::Normal, show:  true,
        name:  "SCE", short: "SysCall", long:  "System call extensions",
    },
    BitSpan {
        span:  RangeInclusive::new(1, 1),
        kind:  BitSpanKind::Normal, show:  true,
        name:  "LME", short: "Long mode enable", long:  "Long mode enable",
    },
    BitSpan {
        span:  RangeInclusive::new(10, 10),
        kind:  BitSpanKind::Normal, show:  true,
        name:  "LMA", short: "Long mode active", long:  "Long mode active",
    },
    BitSpan {
        span:  RangeInclusive::new(11, 11),
        kind:  BitSpanKind::Normal, show:  true,
        name:  "NXE", short: "No-execute enable", long:  "No-execute enable",
    },
    BitSpan {
        span:  RangeInclusive::new(12, 12),
        kind:  BitSpanKind::Normal, show:  true,
        name:  "SVME", short: "SVM enable", long:  "Secure virtual machine enable (AMD)",
    },
    BitSpan {
        span:  RangeInclusive::new(13, 13),
        kind:  BitSpanKind::Normal, show:  true,
        name:  "LMSL", short: "LMSL enable", long:  "Long mode segment limit enable (AMD)",
    },
    BitSpan {
        span:  RangeInclusive::new(14, 14),
        kind:  BitSpanKind::Normal, show:  true,
        name:  "FFXSR", short: "Fast FXSAVE/FXRSTOR (AMD)", long:  "Fast FXSAVE/FXRSTOR (AMD)",
    },
];

pub static CPU_X86_CR0_BITS: [BitSpan; 11] = [
    BitSpan {
        span:  RangeInclusive::new(0, 0),
        kind:  BitSpanKind::Normal, show:  true,
        name:  "PE", short: "PE", long:  "Protected mode enable",
    },
    BitSpan {
        span:  RangeInclusive::new(1, 1),
        kind:  BitSpanKind::Normal, show:  true,
        name:  "MP", short: "MP", long:  "Monitor co-processor",
    },
    BitSpan {
        span:  RangeInclusive::new(2, 2),
        kind:  BitSpanKind::Normal, show:  true,
        name:  "EM", short: "EM", long:  "x87 FPU emulation",
    },
    BitSpan {
        span:  RangeInclusive::new(3, 3),
        kind:  BitSpanKind::Normal, show:  true,
        name:  "TS", short: "TS", long:  "Task switched",
    },
    BitSpan {
        span:  RangeInclusive::new(4, 4),
        kind:  BitSpanKind::Normal, show:  true,
        name:  "ET", short: "ET", long:  "Extension type",
    },
    BitSpan {
        span:  RangeInclusive::new(5, 5),
        kind:  BitSpanKind::Normal, show:  true,
        name:  "NE", short: "NE", long:  "Numeric error",
    },
    BitSpan {
        span:  RangeInclusive::new(16, 16),
        kind:  BitSpanKind::Normal, show:  true,
        name:  "WP", short: "WP", long:  "Write protect",
    },
	BitSpan {
		span:  RangeInclusive::new(18, 18),
		kind:  BitSpanKind::Normal, show:  true,
		name:  "AM", short: "AM", long:  "Alignment mask",
	},
	BitSpan {
		span:  RangeInclusive::new(29, 29),
		kind:  BitSpanKind::Normal, show:  true,
		name:  "NW", short: "NW", long:  "No-write through",
	},
	BitSpan {
		span:  RangeInclusive::new(30, 30),
		kind:  BitSpanKind::Normal, show:  true,
		name:  "CD", short: "CD", long:  "Cache disable",
	},
	BitSpan {
		span:  RangeInclusive::new(31, 31),
		kind:  BitSpanKind::Normal, show:  true,
		name:  "PG", short: "PG", long:  "Paging",
	},
];

*/
