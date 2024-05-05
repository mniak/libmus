
REM Compile and Link
REM You need to compile the libmus using 'cargo build --release' first
SET NATIVE_STATIC_LIBS=advapi32.lib bcrypt.lib kernel32.lib msvcrt.lib ntdll.lib userenv.lib ws2_32.lib
cl main.c libmus.dll.lib ^
   /MD ^
   /I../../headers ^
   /link /LIBPATH:..\..\target\release %NATIVE_STATIC_LIBS%

copy ..\..\target\release\libmus.dll .