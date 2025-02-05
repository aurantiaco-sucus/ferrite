
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TargetArch {
    X86,
    X86_64,
    Arm,
    AArch64,
    RiscV32,
    RiscV64,
    Mips,
    Loong64,
}

#[cfg(target_arch = "x86")]
pub const TARGET_ARCH: TargetArch = TargetArch::X86;
#[cfg(target_arch = "x86_64")]
pub const TARGET_ARCH: TargetArch = TargetArch::X86_64;
#[cfg(target_arch = "arm")]
pub const TARGET_ARCH: TargetArch = TargetArch::Arm;
#[cfg(target_arch = "aarch64")]
pub const TARGET_ARCH: TargetArch = TargetArch::AArch64;
#[cfg(target_arch = "riscv32")]
pub const TARGET_ARCH: TargetArch = TargetArch::RiscV32;
#[cfg(target_arch = "riscv64")]
pub const TARGET_ARCH: TargetArch = TargetArch::RiscV64;
#[cfg(target_arch = "mips")]
pub const TARGET_ARCH: TargetArch = TargetArch::Mips;
#[cfg(target_arch = "loongarch64")]
pub const TARGET_ARCH: TargetArch = TargetArch::Loong64;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TargetSystem {
    Windows,
    Linux,
    MacOS,
    Android,
    IOS,
    FreeBSD,
    DragonFly,
    NetBSD,
    OpenBSD,
    BareMetal,
}

#[cfg(target_os = "windows")]
pub const TARGET_SYSTEM: TargetSystem = TargetSystem::Windows;
#[cfg(target_os = "linux")]
pub const TARGET_SYSTEM: TargetSystem = TargetSystem::Linux;
#[cfg(target_os = "macos")]
pub const TARGET_SYSTEM: TargetSystem = TargetSystem::MacOS;
#[cfg(target_os = "android")]
pub const TARGET_SYSTEM: TargetSystem = TargetSystem::Android;
#[cfg(target_os = "ios")]
pub const TARGET_SYSTEM: TargetSystem = TargetSystem::IOS;
#[cfg(target_os = "freebsd")]
pub const TARGET_SYSTEM: TargetSystem = TargetSystem::FreeBSD;
#[cfg(target_os = "dragonfly")]
pub const TARGET_SYSTEM: TargetSystem = TargetSystem::DragonFly;
#[cfg(target_os = "netbsd")]
pub const TARGET_SYSTEM: TargetSystem = TargetSystem::NetBSD;
#[cfg(target_os = "openbsd")]
pub const TARGET_SYSTEM: TargetSystem = TargetSystem::OpenBSD;
#[cfg(target_os = "none")]
pub const TARGET_SYSTEM: TargetSystem = TargetSystem::BareMetal;