### Install Rust

#### Linux&Mac

打开终端输入以下命令

```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

此命令包含一个脚本，该脚本将安装`rustup`工具，此工具将安装Rust最新稳定版本。可能会提示您输入管理员密码。

安装成功

```bash
Rust is installed now. Great!
```

#### 安装C语言编译器

Rust 对运行环境的依赖和 Go 语言很像，几乎所有环境都可以无需安装任何依赖直接运行。但是，Rust 会依赖 libc 和链接器
linker。所以如果遇到了提示链接器无法执行的错误，你需要再手动安装一个 C 语言编译器

MacOs

```bash
$ xcode-select --install
```

Linux 用户一般应按照相应发行版的文档来安装 GCC 或 Clang。

例如，如果你使用 Ubuntu，则可安装 `build-essential`。

#### Windows

先安装 Microsoft C++ Build Tools [https://visualstudio.microsoft.com/zh-hans/visual-cpp-build-tools/] , 勾选安装 C++
环境即可。安装时可自行修改缓存路径与安装路径，避免占用过多 C 盘空间。安装完成后，Rust 所需的 msvc 命令行程序需要手动添加到环境变量中，否则安装
Rust 时 rustup-init 会提示未安装 Microsoft C++ Build Tools，其位于：%Visual Studio
安装位置%\VC\Tools\MSVC\%version%\bin\Hostx64\x64（请自行替换其中的 %Visual Studio 安装位置%、%version% 字段）下。

如果你不想这么做，可以选择安装 Microsoft C++ Build Tools 新增的“定制”终端 Developer Command Prompt for %Visual Studio
version% 或 Developer PowerShell for %Visual Studio version%，在其中运行 rustup-init.exe。

准备好 C++ 环境后开始安装 Rust：

https://www.rust-lang.org/learn/get-started

在 RUSTUP-INIT 下载系统相对应的 Rust 安装程序，一路默认即可。

#### Uninstall

```bash
$ rustup self uninstall
```

#### 检查安装是否成功

```bash
$ rustc -V
rustc 1.56.1 (59eed8a2a 2021-11-01)

$ cargo -V
cargo 1.57.0 (b2e52d7ca 2021-10-21)
```

#### 本地文档

```bash
$ rustup doc
```



