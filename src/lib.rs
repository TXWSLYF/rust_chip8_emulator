//! CHIP-8 模拟核心：内存、CPU 与显示。窗口、输入与计时应由宿主（桌面或浏览器）实现。

pub mod constants;
pub mod cpu;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::WasmEmulator;
