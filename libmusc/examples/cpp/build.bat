cl /c main.c

link /OUT:main.exe libmusc.lib /MACHINE:X64 /LIBPATH:../../target/release main.obj