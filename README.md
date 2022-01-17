# rtea
Bring fast TEA cipher to Python. Powered by Rust.

为 Python 带来快速的TEA密码实现

## 依赖
### 开发依赖
1. ~~一个能跑起来的 Rust，不知道最低是什么版本，但是用新的准没错~~ 你大概需要一个 Nightly 的 Rust
2. Python 3.7+
3. `requirements-dev.txt` 里的 `maturin`

### 运行依赖
1. Python 3.7+

## 构建

假设环境已经存在满足条件的 Rust 和 Python

1. 新建一个虚拟环境并激活
2. 安装开发依赖 `pip install -r requirements-dev.txt`
3. 运行 `maturin develop [--release]` 在 `rtea` 目录下生成二进制库文件，可直接引入
4. 或者运行 `maturin build [--release]` 在 `target/wheels` 下生成 `whl`，在 Unix 系统上还可添加 `--strip` 以减少生成体积

## 测试

1. 运行 `pytest` 同时运行单元测试与速度测试
2. 运行 `cargo test` 运行单元测试
2. 运行 `cargo bench` 运行速度测试

## 预编译包
1. 从右侧 Release 页面获取
2. 使用 pip 直接从 PyPI 安装
