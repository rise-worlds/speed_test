# SpeedTest for speedtest.cn
`rust`版命令行测试工具，支持[测速网](https://www.speedtest.cn/)

## 编译

### windows 配置预编译环境
- 安装 [Visual Studio C/C++ Build Tools](https://aka.ms/vs/17/release/vs_BuildTools.exe) 
- 安装 [CMake](https://cmake.org/download/)，并添加到环境变量。
- 安装 [NASM](https://nasm.us/)，并添加到环境变量。
- 安装 [Ninja](https://github.com/ninja-build/ninja/releases)，并添加到环境变量。
- 安装 [Clang/LLVM](https://github.com/llvm/llvm-project/releases/)，并添加到环境变量。

### windows 配置 `openssl`
使用`native-tls`需要安装`openssl`库

- 安装配置[Win32OpenSSL](https://slproweb.com/products/Win32OpenSSL.html)
- 或者使用`vcpkg`安装
```shell
#安装vcpkg和openssl
cd F:\Github
git clone https://github.com/microsoft/vcpkg
cd vcpkg
./bootstrap-vcpkg.bat
./vcpkg install openssl:x64-windows-static

#设置环境变量
$env:OPENSSL_DIR='%VCPKG_ROOT%\installed\x64-windows-static'
$env:OPENSSL_INCLUDE_DIR="%VCPKG_ROOT%\installed\x64-windows-static\include"
$env:OPENSSL_LIB_DIR="%VCPKG_ROOT%\installed\x64-windows-static\lib"
$env:OPENSSL_STATIC='Yes'
$env:OPENSSL_NO_VENDOR=1

[System.Environment]::SetEnvironmentVariable('OPENSSL_DIR', $env:OPENSSL_DIR, [System.EnvironmentVariableTarget]::Machine)
[System.Environment]::SetEnvironmentVariable('OPENSSL_STATIC', $env:OPENSSL_STATIC, [System.EnvironmentVariableTarget]::Machine)
[System.Environment]::SetEnvironmentVariable('OPENSSL_NO_VENDOR', $env:OPENSSL_NO_VENDOR, [System.EnvironmentVariableTarget]::Machine)
[System.Environment]::SetEnvironmentVariable('OPENSSL_INCLUDE_DIR', $env:OPENSSL_INCLUDE_DIR, [System.EnvironmentVariableTarget]::Machine)
[System.Environment]::SetEnvironmentVariable('OPENSSL_LIB_DIR', $env:OPENSSL_LIB_DIR, [System.EnvironmentVariableTarget]::Machine)

#安装choco和make
Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
choco install make -y

```
