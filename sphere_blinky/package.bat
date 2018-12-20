@echo off
setlocal
set AZSPHERETOOLS=c:\Program Files (x86)\Microsoft Azure Sphere SDK\Tools\
set PATH=%PATH%;%AZSPHERETOOLS%

mkdir target\approot\bin
copy target\arm-v7-none-eabi\debug\blinky_sphere target\approot\bin\app
copy app_manifest.json target\approot

azsphere image package-application --input target\approot --output target\manual.imagepackage --sysroot 1 -v

echo "Now do this:"
echo "azsphere device sideload delete"
echo "azsphere device sideload deploy -p manual.imagepackage"
echo

