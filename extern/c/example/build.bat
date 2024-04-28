cl /c main.c

link /OUT:main.exe libmus.lib /MACHINE:X64 /LIBPATH:../../../target/release main.obj