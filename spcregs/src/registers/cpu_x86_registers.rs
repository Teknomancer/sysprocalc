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


