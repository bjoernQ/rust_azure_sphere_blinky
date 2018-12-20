use std::env;
use std::process::Command;
use std::process;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut args_to_gcc: Vec<String> = Vec::new();
    let mut idx = 0;
    let mut take_next = false;

    for arg in args {
        if idx == 0 {
            idx = idx + 1;
            continue;
        }

        if take_next {
            take_next = false;
            args_to_gcc.push(arg.clone());
        }

        if arg == "-L" {
            args_to_gcc.push(arg.clone());
            take_next = true;
            idx = idx +1;
            continue;
        }

        if arg == "-o" {
            args_to_gcc.push(arg.clone());
            take_next = true;
            idx = idx +1;
            continue;
        }

        if arg.ends_with(".o") {
            args_to_gcc.push(arg.clone());
        }

        idx = idx +1;
    };

    args_to_gcc.push("--sysroot=C:\\Program Files (x86)\\Microsoft Azure Sphere SDK\\Sysroots\\1".to_string());
    args_to_gcc.push("-Wl,--no-undefined".to_string());
    args_to_gcc.push("-nodefaultlibs".to_string());
    args_to_gcc.push("-B".to_string());
    args_to_gcc.push("C:\\Program Files (x86)\\Microsoft Azure Sphere SDK\\Sysroots\\1\\tools\\gcc".to_string());
    args_to_gcc.push("-march=armv7ve".to_string());
    args_to_gcc.push("-mcpu=cortex-a7".to_string());
    args_to_gcc.push("-mthumb".to_string());
    args_to_gcc.push("-mfpu=neon".to_string());
    args_to_gcc.push("-mfloat-abi=hard".to_string());
    args_to_gcc.push("-lapplibs".to_string());
    args_to_gcc.push("-lpthread".to_string());
    args_to_gcc.push("-lgcc_s".to_string());
    args_to_gcc.push("-lc".to_string());

    eprintln!("args to gcc {:?}", args_to_gcc);

    let output =  Command::new("C:\\Program Files (x86)\\Microsoft Azure Sphere SDK\\Sysroots\\1\\tools\\gcc\\gcc.exe")
            .args(&args_to_gcc)
            .output()
            .expect("failed to execute process");

    let out = output.stdout;
    //println!("{}", out);

    let err_out = str::from_utf8(&(output.stderr)).expect("unable to get stderr");
    eprintln!("{}", err_out);

    process::exit(0);
}

/*
setlocal
set AZSPHERETOOLS=c:\Program Files (x86)\Microsoft Azure Sphere SDK\Tools\
set SYSROOT=C:\Program Files (x86)\Microsoft Azure Sphere SDK\Sysroots\1
set CCPATH=%SYSROOT%\tools\gcc
set CLANGPATH=D:\.konan\dependencies\msys2-mingw-w64-x86_64-gcc-7.3.0-clang-llvm-lld-6.0.1\bin
set PATH=%PATH%;%CCPATH%;%CLANGPATH%;%AZSPHERETOOLS%

gcc -o "Blink.out" --sysroot="%SYSROOT%" -Wl,--no-undefined -nodefaultlibs -B "%SYSROOT%\tools\gcc" -march=armv7ve -mcpu=cortex-a7 -mthumb -mfpu=neon -mfloat-abi=hard hello.o -lapplibs -lpthread -lgcc_s -lc
gcc -o "Blink.out" hello.o --sysroot="%SYSROOT%" -Wl,--no-undefined -nodefaultlibs -B "%SYSROOT%\tools\gcc" -march=armv7ve -mcpu=cortex-a7 -mthumb -mfpu=neon -mfloat-abi=hard -lapplibs -lpthread -lgcc_s -lc


-o "Blink.out"
hello.o

--sysroot="%SYSROOT%"
-Wl,--no-undefined
-nodefaultlibs
-B "%SYSROOT%\tools\gcc"
-march=armv7ve
-mcpu=cortex-a7
-mthumb
-mfpu=neon
-mfloat-abi=hard
-lapplibs
-lpthread
-lgcc_s
-lc


*/


/*
 = note: "cmd" "/c" "link.bat" "-L" "C:\\Users\\bjoern\\.xargo\\lib\\rustlib\\arm-v7-none-eabi\\lib" "D:\\workspaces\\test\\target\\arm-v7-none-eabi\\debug\\deps\\BlinkySphere-e957dd655fdee2c4.zzrb1uug1y5px4t.rcgu.o" "-o" "D:\\workspaces\\test\\target\\arm-v7-none-eabi\\debug\\deps\\BlinkySphere-e957dd655fdee2c4" "-Wl,--gc-sections" "-nodefaultlibs" "-L" "D:\\workspaces\\test\\target\\arm-v7-none-eabi\\debug\\deps" "-L" "D:\\workspaces\\test\\target\\debug\\deps" "-L" "C:\\Users\\bjoern\\.xargo\\lib\\rustlib\\arm-v7-none-eabi\\lib" "-Wl,-Bstatic" "C:\\Users\\bjoern\\.xargo\\lib\\rustlib\\arm-v7-none-eabi\\lib\\libcore-79b7ca805d23dd23.rlib" "C:\\Users\\bjoern\\.xargo\\lib\\rustlib\\arm-v7-none-eabi\\lib\\libcompiler_builtins-5974d7451bf4ffc6.rlib" "-Wl,-Bdynamic"
  = note: lala -L C:\Users\bjoern\.xargo\lib\rustlib\arm-v7-none-eabi\lib D:\workspaces\test\target\arm-v7-none-eabi\debug\deps\BlinkySphere-e957dd655fdee2c4.zzrb1uug1y5px4t.rcgu.o -o D:\workspaces\test\target\arm-v7-none-eabi\debug\deps\BlinkySphere-e957dd655fdee2c4 -Wl,--gc-sections -nodefaultlibs -L D:\workspaces\test\target\arm-v7-none-eabi\debug\deps -L D:\workspaces\test\target\debug\deps -L C:\Users\bjoern\.xargo\lib\rustlib\arm-v7-none-eabi\lib -Wl,-Bstatic C:\Users\bjoern\.xargo\lib\rustlib\arm-v7-none-eabi\lib\libcore-79b7ca805d23dd23.rlib C:\Users\bjoern\.xargo\lib\rustlib\arm-v7-none-eabi\lib\libcompiler_builtins-5974d7451bf4ffc6.rlib -Wl,-Bdynamic
          args -L
          args C:\Users\bjoern\.xargo\lib\rustlib\arm-v7-none-eabi\lib
          args D:\workspaces\test\target\arm-v7-none-eabi\debug\deps\BlinkySphere-e957dd655fdee2c4.zzrb1uug1y5px4t.rcgu.o
          args -o
          args D:\workspaces\test\target\arm-v7-none-eabi\debug\deps\BlinkySphere-e957dd655fdee2c4
          args -Wl
          args --gc-sections
          args -nodefaultlibs
          args -L
          args D:\workspaces\test\target\arm-v7-none-eabi\debug\deps
          args -L
          args D:\workspaces\test\target\debug\deps
          args -L
          args C:\Users\bjoern\.xargo\lib\rustlib\arm-v7-none-eabi\lib
          args -Wl
          args -Bstatic
          args C:\Users\bjoern\.xargo\lib\rustlib\arm-v7-none-eabi\lib\libcore-79b7ca805d23dd23.rlib
          args C:\Users\bjoern\.xargo\lib\rustlib\arm-v7-none-eabi\lib\libcompiler_builtins-5974d7451bf4ffc6.rlib
          args -Wl
          args -Bdynamic
          Datei nicht gefunden
          Datei nicht gefunden
          Datei nicht gefunden
          Datei nicht gefunden
          Datei nicht gefunden
          Datei nicht gefunden
          Datei nicht gefunden
          Datei nicht gefunden
          Datei nicht gefunden
          Datei nicht gefunden
          Datei nicht gefunden
          Datei nicht gefunden
          Datei nicht gefunden

          "\n"
          compiler_builtins-5974d7451bf4ffc6.d
          core-79b7ca805d23dd23.d
          libcompiler_builtins-5974d7451bf4ffc6.rlib
          libcore-79b7ca805d23dd23.rlib

          "\n"
          BlinkySphere-e957dd655fdee2c4.zzrb1uug1y5px4t.rcgu.o

          "\n"

          "\n"

          "\n"

          "\n"

          "\n"

          "\n"

          "\n"
          BlinkySphere-e957dd655fdee2c4.25htlwio5jnie0pp.rcgu.o
          BlinkySphere-e957dd655fdee2c4.d
          BlinkySphere-e957dd655fdee2c4.zzrb1uug1y5px4t.rcgu.o

          "\n"

          "\n"

          "\n"

          "\n"
          compiler_builtins-5974d7451bf4ffc6.d
          core-79b7ca805d23dd23.d
          libcompiler_builtins-5974d7451bf4ffc6.rlib
          libcore-79b7ca805d23dd23.rlib

          "\n"

          "\n"

          "\n"
          libcore-79b7ca805d23dd23.rlib

          "\n"
          libcompiler_builtins-5974d7451bf4ffc6.rlib

          "\n"

          "\n"

          "\n"


"-L" "C:\Users\bjoern\.xargo\lib\rustlib\arm-v7-none-eabi\lib" "D:\workspaces\test\target\arm-v7-none-eabi\debug\deps\BlinkySphere-e957dd655fdee2c4.zzrb1uug1y5px4t.rcgu.o" "-o" "D:\workspaces\test\target\arm-v7-none-eabi\debug\deps\BlinkySphere-e957dd655fdee2c4" "-L" "D:\workspaces\test\target\arm-v7-none-eabi\debug\deps" "-L" "D:\workspaces\test\target\debug\deps" "-L" "C:\Users\bjoern\.xargo\lib\rustlib\arm-v7-none-eabi\lib" "--sysroot=\"C:\Program Files (x86)\Microsoft Azure Sphere SDK\Sysroots\1\"" "-Wl,--no-undefined" "-nodefaultlibs" "-B \"C:\Program Files (x86)\Microsoft Azure Sphere SDK\Sysroots\1\tools\gcc\"" "-march=armv7ve" "-mcpu=cortex-a7" "-mthumb" "-mfpu=neon" "-mfloat-abi=hard" "-lapplibs" "-lpthread" "-lgcc_s" "-lc"

"%CCPATH%\gcc.exe" -L "C:\Users\bjoern\.xargo\lib\rustlib\arm-v7-none-eabi\lib" "D:\workspaces\test\target\arm-v7-none-eabi\debug\deps\BlinkySphere-e957dd655fdee2c4.zzrb1uug1y5px4t.rcgu.o" -o "D:\workspaces\test\target\arm-v7-none-eabi\debug\deps\BlinkySphere-e957dd655fdee2c4" "-L" "D:\workspaces\test\target\arm-v7-none-eabi\debug\deps" "-L" "D:\workspaces\test\target\debug\deps" "-L" "C:\Users\bjoern\.xargo\lib\rustlib\arm-v7-none-eabi\lib" --sysroot="C:\Program Files (x86)\Microsoft Azure Sphere SDK\Sysroots\1\" "-Wl,--no-undefined" "-nodefaultlibs" -B "C:\Program Files (x86)\Microsoft Azure Sphere SDK\Sysroots\1\tools\gcc\"-march=armv7ve" "-mcpu=cortex-a7" "-mthumb" "-mfpu=neon" "-mfloat-abi=hard" "-lapplibs" "-lpthread" "-lgcc_s" "-lc"

"%CCPATH%\gcc.exe" -L "C:\Users\bjoern\.xargo\lib\rustlib\arm-v7-none-eabi\lib" "D:\workspaces\test\target\arm-v7-none-eabi\debug\deps\BlinkySphere-e957dd655fdee2c4.zzrb1uug1y5px4t.rcgu.o" -o "D:\workspaces\test\target\arm-v7-none-eabi\debug\deps\BlinkySphere-e957dd655fdee2c4" "-L" "D:\workspaces\test\target\arm-v7-none-eabi\debug\deps" "-L" "D:\workspaces\test\target\debug\deps" "-L" "C:\Users\bjoern\.xargo\lib\rustlib\arm-v7-none-eabi\lib" --sysroot="C:\Program Files (x86)\Microsoft Azure Sphere SDK\Sysroots\1" "-Wl,--no-undefined" "-nodefaultlibs" -B "C:\Program Files (x86)\Microsoft Azure Sphere SDK\Sysroots\1\tools\gcc" -march=armv7ve "-mcpu=cortex-a7" "-mthumb" "-mfpu=neon" "-mfloat-abi=hard" "-lapplibs" "-lpthread" "-lgcc_s" "-lc"
"%CCPATH%\gcc.exe" -L "C:\Users\bjoern\.xargo\lib\rustlib\arm-v7-none-eabi\lib" "D:\workspaces\test\target\arm-v7-none-eabi\debug\deps\BlinkySphere-e957dd655fdee2c4.zzrb1uug1y5px4t.rcgu.o" -o "D:\workspaces\test\target\arm-v7-none-eabi\debug\deps\BlinkySphere-e957dd655fdee2c4" "-L" "D:\workspaces\test\target\arm-v7-none-eabi\debug\deps" "-L" "D:\workspaces\test\target\debug\deps" "-L" "C:\Users\bjoern\.xargo\lib\rustlib\arm-v7-none-eabi\lib" --sysroot="C:\Program Files (x86)\Microsoft Azure Sphere SDK\Sysroots\1" "-Wl,--no-undefined" "-nodefaultlibs" -B "C:\Program Files (x86)\Microsoft Azure Sphere SDK\Sysroots\1\tools\gcc" -march=armv7ve "-mcpu=cortex-a7" "-mthumb" "-mfpu=neon" "-mfloat-abi=hard" "-lapplibs" "-lpthread" "-lgcc_s" "-lc"


"%CCPATH%\gcc" -o "Blink.out" hello.o --sysroot="C:\Program Files (x86)\Microsoft Azure Sphere SDK\Sysroots\1" -Wl,--no-undefined -nodefaultlibs -B "C:\Program Files (x86)\Microsoft Azure Sphere SDK\Sysroots\1\tools\gcc" -march=armv7ve -mcpu=cortex-a7 -mthumb -mfpu=neon -mfloat-abi=hard -lapplibs -lpthread -lgcc_s -lc

"%CCPATH%\gcc" -L "C:\Users\bjoern\.xargo\lib\rustlib\arm-v7-none-eabi\lib" hello.o -o "Blink.out"  --sysroot="C:\Program Files (x86)\Microsoft Azure Sphere SDK\Sysroots\1" -Wl,--no-undefined -nodefaultlibs -B "C:\Program Files (x86)\Microsoft Azure Sphere SDK\Sysroots\1\tools\gcc" -march=armv7ve -mcpu=cortex-a7 -mthumb -mfpu=neon -mfloat-abi=hard -lapplibs -lpthread -lgcc_s -lc

*/