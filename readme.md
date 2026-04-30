# rust_chip8_emulator

用 Rust 实现的 CHIP-8 模拟器：核心为平台无关库，桌面端使用 [minifb](https://github.com/emoon/minifb)，浏览器端通过 **WebAssembly** + `wasm-bindgen` 运行。

## 环境要求

- **Rust**（建议已安装 [rustup](https://rustup.rs/)）
- 桌面版：默认启用 `native` feature，无需额外配置
- 网页版：
  - 目标三元组：`wasm32-unknown-unknown`（`rustup target add wasm32-unknown-unknown`）
  - [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)（用于生成 `web/pkg` 下的 JS 胶水代码）

## 项目结构

| 路径 | 说明 |
|------|------|
| `src/lib.rs` | 库入口，导出 `constants`、`cpu` |
| `src/cpu.rs` | CPU 与指令实现 |
| `src/constants.rs` | 内存、分辨率、字体等常量 |
| `src/main.rs` | 桌面可执行文件：窗口、键盘、计时、渲染 |
| `src/wasm.rs` | 仅在 `wasm32` 下编译，导出 `WasmEmulator` |
| `web/index.html` | 浏览器 UI：选 ROM、`canvas`、键位 |
| `web/pkg/` | 由 `wasm-pack` 生成（`.wasm` + `.js`），缺失时需自行构建 |

## 桌面运行

编译并运行（二进制名为 **`chip8`**）：

```bash
cargo run --release --bin chip8 -- path/to/game.ch8
```

第一个参数为 ROM 文件路径。窗口内 **Esc** 退出。

### 键位（与 CHIP-8 十六键对应）

| 键盘 | CHIP-8 |
|------|--------|
| 1 2 3 4 | 1 2 3 C |
| Q W E R | 4 5 6 D |
| A S D F | 7 8 9 E |
| Z X C V | A 0 B F |

## 仅构建核心库（无 minifb）

关闭默认的 `native` feature 时不会编译桌面二进制，也不会链接 `minifb`：

```bash
cargo build --no-default-features
```

## 网页 / WASM

1. 安装 wasm 目标与 wasm-pack（若尚未安装）：

   ```bash
   rustup target add wasm32-unknown-unknown
   # wasm-pack 见官方安装说明
   ```

2. 在项目根目录生成浏览器产物：

   ```bash
   wasm-pack build --target web --no-default-features --out-dir web/pkg
   ```

3. **务必通过本地 HTTP 服务**打开页面（避免 `file://` 下 ES module / Wasm 加载失败），例如：

   ```bash
   cd web
   python3 -m http.server 8080
   ```

   浏览器访问 `http://localhost:8080`，选择 ROM 后开始运行。键位与桌面版相同。

## 开发说明

- 模拟循环：每帧执行若干条 `tick()`，延迟/声音计时器按约 **60 Hz** 调用 `tick_timers()`（与常见 CHIP-8 实现一致）。
- 部分 opcode 可能尚未实现；遇到未支持指令时程序会 `panic` / `todo`，换 ROM 或补全指令即可。

## 许可证

未在仓库中声明许可证时，默认保留所有权利；如需开源请自行添加 `LICENSE` 并在 `Cargo.toml` 中填写 `license` 字段。
