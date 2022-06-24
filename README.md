



cargo build --target x86_64-lin_os.json
cargo xbuild --target x86_64-lin_os.json
cargo xbuild

cargo bootimage
    会使用cargo xbuild编译bootloader 以及我们的内核之后 然后把他们组合在一起


注意安装低版本的qemu--我安装了2015年的
https://qemu.weilnetz.de/w64/2015/qemu-w64-setup-20150115.exe

cargo xrun

cargo xtest