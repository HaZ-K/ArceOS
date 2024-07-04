cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
        pub use self::x86_64::setup_gpm;
    } else if #[cfg(target_arch = "riscv64")] {
        mod riscv64;
        pub use self::riscv64::setup_gpm;
    }
}