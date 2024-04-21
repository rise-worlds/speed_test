# Speed Test


## 编译

### windows 配置openssl

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
