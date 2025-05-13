@ECHO OFF
ECHO:
ECHO "eamake.cmd" is a script which builds EuroAssembler in MS Windows.
ECHO It expects that the downloaded "euroasm.zip" is unzipped in directory
ECHO  whose symbolic name is marked here as "%%EUROASM%%".
IF NOT EXIST "..\euroasm.exe" ECHO Run this script in the directory "%%EUROASM%%\easource". && GOTO :EOF
ECHO The script takes modules from the directory "%%EUROASM%%\easource\*.htm",
ECHO  uses "%%EUROASM%%\euroasm.exe" to assemble them into COFF format "%%EUROASM%%\easource\*.obj"
ECHO  and then it uses the same executable file "%%EUROASM%%\euroasm.exe" to link the modules
ECHO  into target executable file "%%EUROASM%%\easource\euroasm.exe".
ECHO Optional argument of this script may look like "1512345678"
ECHO  and it specifies the nominal build TimeStamp (important for source audit).
ECHO:
ECHO For building EuroAssembler in Linux use the script "eamake.sh".
ECHO:
ECHO Press Enter key to start the rebuild.
PAUSE
ECHO Assembly of EuroAssembler modules begins...
ECHO:
ECHO:>euroasm.dat && REM  Create a temporary file "euroasm.dat" with names of all source modules (without  file extension).
FOR %%m IN (coffstub ea src pgm pass eaopt pgmopt chunk ctx dict exp lst )                                     DO ECHO %%m >>euroasm.dat
FOR %%m IN (mac member msg pseudo reloc sss stm sym var)                                                       DO ECHO %%m >>euroasm.dat
FOR %%m IN (ii iia iib iic iid iif iig iik iim iip iis iit iiv iix iiy iiz)                                    DO ECHO %%m >>euroasm.dat
FOR %%m IN (pf pfbin pfboot pfcom pfomf pflibomf pfmz pfcoff pflibcof pfpe pfdll pfrsrc pfelf pfelfx pfelfso)  DO ECHO %%m >>euroasm.dat
FOR /F %%m IN (euroasm.dat) DO (..\euroasm.exe %%m.htm TimeStamp=%1 && ECHO:)
ECHO:
ECHO Linking EuroAssembler modules ...
ECHO:
..\euroasm.exe euroasm.htm TimeStamp=%1
ECHO:
ECHO euroasm.exe was created in this directory "easource".
DIR euroasm.exe