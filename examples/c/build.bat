@echo off

REM Compile and Link
REM You need to compile the libmusc using 'cargo build --release' first
cl main.c /I../../headers /link /LIBPATH:../../target/release libmusc.lib