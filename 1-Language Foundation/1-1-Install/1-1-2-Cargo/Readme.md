### Cargo

cargo 是Rust官方推出的包管理工具，其效用等同于Node的npm、Python的pip、Java的maven等。

#### New Project

```bash
$ cargo new world_hello  ##cargo new world_hello --bin //早期的 cargo 在创建项目时，必须添加 --bin 的参数
$ cd world_hello
```

除了bin类型的项目，还有lib类型的项目，lib类型的项目可以被其他项目引用，类似于Java的jar包。

### Run Project

直接运行

```bash
$ cargo run
```

手动编译后运行

```bash
$ cargo build
$ ./target/debug/world_hello  ##默认的编译产物是在debug模式下的，编译器不会对代码进行优化
```

编译并优化

```bash
$ cargo build --release
$ ./target/release/world_hello  ##release模式下的编译产物，编译器会对代码进行优化
```

### Cargo Check

当项目体量变大时，编译时间会变长，可以使用cargo check来检查代码是否能够编译通过，而不进行编译，从而节省时间。

```bash
$ cargo check
```

### Cargo.toml & Cargo.lock

Cargo.toml和Cargo.lock是cargo的配置文件，类似于npm的package.json和package-lock.json。

- Cargo.toml 是 cargo 特有的项目数据描述文件。它存储了项目的所有元配置信息，如果 Rust 开发者希望 Rust 项目能够按照期望的方式进行构建、测试和运行，那么，必须按照合理的方式构建 Cargo.toml。

- Cargo.lock 文件是 cargo 工具根据同一项目的 toml 文件生成的项目依赖详细清单，因此我们一般不用修改它，只需要对着 Cargo.toml 文件撸就行了。

[package]

描述项目基本信息

```toml
[package]
name = "world_hello"
version = "0.1.0"
edition = "2021"
```

定义项目依赖
在 Cargo.toml 中，主要通过各种依赖段落来描述该项目的各种依赖项：
- 基于 Rust 官方仓库 crates.io，通过版本说明来描述
- 基于项目源代码的 git 仓库地址，通过 URL 来描述
- 基于本地项目的绝对路径或者相对路径，通过类 Unix 模式的路径来描述
```toml
[dependencies]
rand = "0.3" # 从 crates.io 下载
hammer = { version = "0.5.0" } # 从 crates.io 下载
color = { git = "https://github.com/bjz/color-rs" } # 从 git 仓库下载
geometry = { path = "crates/geometry" } # 从本地路径下载
```
