# Speed Test


## build
### config windows env

```shell
#安装vcpkg和openssl
git clone https://github.com/microsoft/vcpkg
cd vcpkg
./bootstrap-vcpkg.bat
./vcpkg install openssl-windows:x64-windows
./vcpkg install openssl:x64-windows-static
./vcpkg.exe integrate install

# 配置环境变量
VCPKG_ROOT=F:\Github\vcpkg
VCPKG_DEFAULT_TRIPLET=x64-windows-release
VCPKG_DEFAULT_HOST_TRIPLET=x64-windows

OPENSSL_NO_VENDOR=1
OPENSSL_STATIC=Yes
OPENSSL_RUST_USE_NASM=0
RUSTFLAGS=-Ctarget-feature=+crt-static
VCPKGRS_DYNAMIC=1
OPENSSL_DIR="%VCPKG_ROOT%\installed\x64-windows-static"
OPENSSL_INCLUDE_DIR="%VCPKG_ROOT%\installed\x64-windows-static\include"
OPENSSL_LIB_DIR="%VCPKG_ROOT%\installed\x64-windows-static\lib"
```
