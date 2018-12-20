# Blinky in Rust for Azure Sphere

Here you can find an example of using Rust to develop Blinky (the "Hello World" of embedded) for Azure Sphere.

## How to build

Make sure to have your Rust environment switched to `nightly` and have xargo installed.

Go to the `sphere_blinky` directory and do `xargo build --target arm-v7-none-eabi` after the build run `package.bat` to get the application packaged for Azure Sphere.

Now you can deploy it to your Sphere dev board via

```
azsphere device sideload delete
azsphere device sideload deploy -p target\manual.imagepackage
```

## Please note

Rust is completely new to me so this might be not the ideal and most idomatic way to do it.

The path to the Azure Sphere SDK is hardcoded and it' assumed to be installed to the default location.
