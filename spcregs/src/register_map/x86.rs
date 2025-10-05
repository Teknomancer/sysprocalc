use crate::RegisterDescriptor;
use std::sync::LazyLock;

pub static CR0: LazyLock<RegisterDescriptor> = LazyLock::new(|| {
    // replace with VarZeroVec::from()?
    toml::from_str(r#"
        arch       = "x86"
        device     = "cpu"
        name       = "cr0"
        desc       = "Control Register 0"
        bit_count  = 32
        byte_order = "LittleEndian"
        bit_ranges = [
            { first=0,  last=0,  kind="Normal", show=true, name="PE",    short="Protected Mode Enable", long="Protected Mode Enable." },
            { first=1,  last=1,  kind="Normal", show=true, name="MP",    short="Monitor Co-Processor",  long="Controls interaction of WAIT/FWAIT instructions with CR0.TS." },
            { first=2,  last=2,  kind="Normal", show=true, name="EM",    short="Emulation",             long="If set, no x87 FPU emulation." },
            { first=3,  last=3,  kind="Normal", show=true, name="TS",    short="Task Switched",         long="If set, save FPU context on FPU instruction after a task switch." },
            { first=4,  last=4,  kind="Normal", show=true, name="ET",    short="Extension Type",        long="On 386, specified if math coprocessor was an 80287 or 80387." },
            { first=5,  last=5,  kind="Normal", show=true, name="NE",    short="Numeric Error",         long="If set, enables x87 FPU error reporting." },
            { first=16, last=16, kind="Normal", show=true, name="WP",    short="Write Protect",         long="Controls whether the CPU can write to pages marked read-only." },
            { first=18, last=18, kind="Normal", show=true, name="AM",    short="Alignment Mask",        long="If set, enables alignment check for CPL=3 when EFLAGS.AC is set." },
            { first=29, last=29, kind="Normal", show=true, name="NW",    short="Not Write-Through",     long="Controls write-back or write-through for writes that hit the cache." },
            { first=30, last=30, kind="Normal", show=true, name="CD",    short="Cache Disable",         long="If set, disables memory caching." },
            { first=31, last=31, kind="Normal", show=true, name="PG",    short="Paging",                long="If set, enables memory paging." },
        ]
    "#).expect("Failed to parse TOML")
});

pub static CR4: LazyLock<RegisterDescriptor> = LazyLock::new(|| {
    toml::from_str(r#"
        arch       = "x86"
        device     = "cpu"
        name       = "cr4"
        desc       = "Control Register 4"
        bit_count  = 32
        byte_order = "LittleEndian"
        bit_ranges = [
            { first=0,  last=0,  kind="Normal", show=true, name="VME",       short="V86 Mode Extensions",                     long="If set, enables virtual-interrupt flag in virtual-8086 mode." },
            { first=1,  last=1,  kind="Normal", show=true, name="PVI",       short="Protected Mode Virtual Interrupts",       long="If set, enables virtual-interrupt flag in protected mode." },
            { first=2,  last=2,  kind="Normal", show=true, name="TSD",       short="TimeStamp Disable",                       long="If set, RDTSC can execute only in ring-0." },
            { first=3,  last=3,  kind="Normal", show=true, name="DE",        short="Debug Extensions",                        long="If set, enables debug register breakpoints on I/O accesses." },
            { first=4,  last=4,  kind="Normal", show=true, name="PSE",       short="Page Size Extension",                     long="If set, enables 4MiB pages in 32-bit paging." },
            { first=5,  last=5,  kind="Normal", show=true, name="PAE",       short="Physical Address Extension",              long="If set, extends 32-bit page tables to translate 36-bit physical address." },
            { first=6,  last=6,  kind="Normal", show=true, name="MCE",       short="Machine Check Exception",                 long="If set, enables machine-check exceptions to occur." },
            { first=7,  last=7,  kind="Normal", show=true, name="PGE",       short="Page Global Enable",                      long="If set, address translation (PDE/PTE) maybe shared between address spaces." },
            { first=8,  last=8,  kind="Normal", show=true, name="PCE",       short="Performance-Monitoring Counter Enable",   long="If set, RDPMC can execute in any privilege level." },
            { first=9,  last=9,  kind="Normal", show=true, name="OSFXSR",    short="OS Support For FXSAVE/FXRSTOR",           long="If set, enables SSE and FXSAVE/FXRSTOR instructions." },
            { first=10, last=10, kind="Normal", show=true, name="OSXMMXCPT", short="OS Support For Unmasked SIMD Exceptions", long="If set, enables unmasked SSE exceptions." },
            { first=11, last=11, kind="Normal", show=true, name="UMIP",      short="User-Mode Instruction Preventiion",       long="If set, #GP on SGDT, SIDT, SLDT, SMSW, STR instructions when CPL > 0." },
            { first=12, last=12, kind="Normal", show=true, name="LA57",      short="57-bit Linear Address",                   long="If set, enables 5-level paging." },
            { first=13, last=13, kind="Normal", show=true, name="VMXE",      short="Virtual Machine Extensions",              long="If set, enables VMX operation. (Intel)" },
            { first=14, last=14, kind="Normal", show=true, name="SMXE",      short="Secure Virtual Machine Extensions",       long="If set, enables SVM operation. (AMD)" },
            { first=16, last=16, kind="Normal", show=true, name="FSGSBASE",  short="FSGSBASE Enable",                         long="If set, enables RDFSBASE, RDGSBASE, WRFSBASE, WRGSBASE instructions." },
            { first=17, last=17, kind="Normal", show=true, name="PCIDE",     short="Process-Context IDs Enable",              long="If set, enables per-process IDs for TLB." },
            { first=18, last=18, kind="Normal", show=true, name="OSXSAVE",   short="OS Support for XSAVE",                    long="If set, enables XSAVE and related instructions." },
            { first=19, last=19, kind="Normal", show=true, name="KL",        short="Key Locker Enable",                       long="If set, enables the AES key locker instructions." },
            { first=20, last=20, kind="Normal", show=true, name="SMEP",      short="Supervisor Mode Execution Prevention",    long="If set, code execution in a higher ring than CPL causes a fault." },
            { first=21, last=21, kind="Normal", show=true, name="SMAP",      short="Supervisor Mode Access Prevention",       long="If set, data access in a higher ring than CPL causes a fault." },
            { first=22, last=22, kind="Normal", show=true, name="PKE",       short="Protection Key Enable",                   long="If set, enables key-based control of user-mode linear addresses and RDPKRU/WRPKRU instructions." },
            { first=23, last=23, kind="Normal", show=true, name="CET",       short="Control-Flow Enforcement Technology",     long="If set, enables CET when CR0.WP IS SET." },
            { first=24, last=24, kind="Normal", show=true, name="PKS",       short="Protection Key For Supervisor Pages",     long="If set, enables protection keys for supervisor pages." },
            { first=25, last=25, kind="Normal", show=true, name="UINTR",     short="User Interrupts",                         long="If set, enables user-mode interrupts." },
            { first=28, last=28, kind="Normal", show=true, name="LAM_SUP",   short="Linear-Address Masking For Supervisor",   long="If set, enables linear-address masking for supervisor pointers." },
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
            { first=0,  last=0,  kind="Normal", show=true, name="SCE",   short="SC Extensions",       long="System Call Extensions." },
            { first=8,  last=8,  kind="Normal", show=true, name="LME",   short="LM Enable",           long="Long Mode Enable." },
            { first=10, last=10, kind="Normal", show=true, name="LMA",   short="LM Active",           long="Long Mode Active." },
            { first=11, last=11, kind="Normal", show=true, name="NXE",   short="NX Enable",           long="No-Execute Enable." },
            { first=12, last=12, kind="Normal", show=true, name="SVME",  short="SVM Enable",          long="Secure Virtual Machine Enable (AMD)." },
            { first=13, last=13, kind="Normal", show=true, name="LMSLE", short="LMSL Enable",         long="Long Mode Segment Limit Enable (AMD)." },
            { first=14, last=14, kind="Normal", show=true, name="FFXSR", short="Fast FXSAVE/FXRSTOR", long="Fast FXSAVE/FXRSTOR support." },
        ]
    "#).expect("Failed to parse TOML")
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BitRange, BitRangeKind, BitSpan, ByteOrder};
    use std::borrow::Cow;

    #[test]
    fn test_x86_cpu_efer() {
        let bits = Vec::from([
            BitRange::new(
                BitSpan::new(0, 0),
                BitRangeKind::Normal,
                true,
                Cow::Borrowed("SCE"),
                Cow::Borrowed("SC Extensions"),
                Cow::Borrowed("System Call Extensions."),
            ),
            BitRange::new(
                BitSpan::new(8, 8),
                BitRangeKind::Normal,
                true,
                Cow::Borrowed("LME"),
                Cow::Borrowed("LM Enable"),
                Cow::Borrowed("Long Mode Enable."),
            ),
            BitRange::new(
                BitSpan::new(10, 10),
                BitRangeKind::Normal,
                true,
                Cow::Borrowed("LMA"),
                Cow::Borrowed("LM Active"),
                Cow::Borrowed("Long Mode Active."),
            ),
            BitRange::new(
                BitSpan::new(11, 11),
                BitRangeKind::Normal,
                true,
                Cow::Borrowed("NXE"),
                Cow::Borrowed("NX Enable"),
                Cow::Borrowed("No-Execute Enable."),
            ),
            BitRange::new(
                BitSpan::new(12, 12),
                BitRangeKind::Normal,
                true,
                Cow::Borrowed("SVME"),
                Cow::Borrowed("SVM Enable"),
                Cow::Borrowed("Secure Virtual Machine Enable (AMD)."),
            ),
            BitRange::new(
                BitSpan::new(13, 13),
                BitRangeKind::Normal,
                true,
                Cow::Borrowed("LMSLE"),
                Cow::Borrowed("LMSL Enable"),
                Cow::Borrowed("Long Mode Segment Limit Enable (AMD)."),
            ),
            BitRange::new(
                BitSpan::new(14, 14),
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
