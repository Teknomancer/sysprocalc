use crate::{BitSpan, BitSpanKind, BitGroup, ByteOrder};
use std::ops::RangeInclusive;

static ARCH: &str = "x86";
static DEVICE: &str = "cpu";

pub static CPU_X86_EFER_BITS: [BitSpan; 7] = [
    BitSpan {
        span:  RangeInclusive::new(0, 0),
        kind:  BitSpanKind::Normal,
        show:  true,
        name:  "SCE",
        short: "SysCall",
        long:  "System Call Extensions",
    },
    BitSpan {
        span:  RangeInclusive::new(1, 1),
        kind:  BitSpanKind::Normal,
        show:  true,
        name:  "LME",
        short: "Long mode enable",
        long:  "Long mode enable",
    },
    BitSpan {
        span:  RangeInclusive::new(10, 10),
        kind:  BitSpanKind::Normal,
        show:  true,
        name:  "LMA",
        short: "Long mode active",
        long:  "Long mode active",
    },
    BitSpan {
        span:  RangeInclusive::new(11, 11),
        kind:  BitSpanKind::Normal,
        show:  true,
        name:  "NXE",
        short: "No-execute enable",
        long:  "No-execute enable",
    },
    BitSpan {
        span:  RangeInclusive::new(12, 12),
        kind:  BitSpanKind::Normal,
        show:  true,
        name:  "SVME",
        short: "SVM enable",
        long:  "Secure virtual machine enable (AMD)",
    },
    BitSpan {
        span:  RangeInclusive::new(13, 13),
        kind:  BitSpanKind::Normal,
        show:  true,
        name:  "LMSL",
        short: "LMSL enable",
        long:  "Long mode segment limit enable (AMD)",
    },
    BitSpan {
        span:  RangeInclusive::new(14, 14),
        kind:  BitSpanKind::Normal,
        show:  true,
        name:  "FFXSR",
        short: "Fast FXSAVE/FXRSTOR (AMD)",
        long:  "Fast FXSAVE/FXRSTOR (AMD)",
    },
];


