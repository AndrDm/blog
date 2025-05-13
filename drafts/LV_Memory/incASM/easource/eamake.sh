#!/bin/bash
echo \"eamake.sh\" is a script which builds EuroAssembler in Linux.
echo It expects that the downloaded \"euroasm.zip\" is unzipped in directory
echo  whose symbolic name is marked here as \$EUROASM.
(ls ../euroasm.x >>  /dev/null 2>&1) || echo ../euroasm.x is missing. Run this script in the directory \"\$EUROASM/easource\"
echo The script takes modules from the directory \"\$EUROASM/easource/*.htm\",
echo  uses \"\$EUROASM/euroasm.x\" to assemble them into COFF format \"\$EUROASM/easource/*.obj\"
echo  and then it uses the same executable file \"\$EUROASM/euroasm.x\" to link the modules
echo  into target executable file \"\$EUROASM/easource/euroasm.x\".
echo .
echo For building EuroAssembler in Windows use the script \"eamake.cmd\".
echo .
read -p "Press Enter key to start the rebuild."
echo .
echo Assembly of EuroAssembler modules begins...
echo .
modules=(coffstub ea src pgm pass eaopt pgmopt chunk ctx dict exp lst mac member msg pseudo reloc sss stm sym var
         ii iia iib iic iid iif iig iik iim iip iis iit iiv iix iiy iiz
         pf pfbin pfboot pfcom pfomf pflibomf pfmz pfcoff pflibcof pfpe pfdll pfrsrc pfelf pfelfx pfelfso)
for module in "${modules[@]}"; do
echo .
../euroasm.x $module.htm
if [ $? -gt 2 ]
then break
fi
done
echo Linking EuroAssembler modules...
../euroasm.x euroasm.htm
echo .
echo "euroasm.x" was created in this directory "easource".
ls -l euroasm.x