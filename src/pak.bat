echo "%~1\*.*" "..\..\..\*.*" >filelist.txt
UnrealPak "%~1.pak" -create=filelist.txt -compress
del filelist.txt
del UnrealPak.exe
del pak.bat