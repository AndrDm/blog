SHELL := cmd.exe
CYGWIN=nontsec
export PATH := C:\Program Files\Python310\Scripts\;C:\Program Files\Python310\;C:\Program Files\Common Files\Oracle\Java\javapath;C:\Program Files\Python313\Scripts\;C:\Program Files\Python313\;C:\Program Files (x86)\DVTk\DVT\bin;C:\Program Files (x86)\Common Files\Intel\Shared Libraries\bin32;C:\Program Files (x86)\Common Files\Intel\Shared Libraries\bin;C:\Program Files\Python312\Scripts\;C:\Program Files\Python312\;C:\Program Files\National Instruments\Shared\OpenVINO\;C:\Program Files (x86)\Embarcadero\Studio\23.0\bin;C:\Users\Public\Documents\Embarcadero\Studio\23.0\Bpl;C:\Program Files (x86)\Embarcadero\Studio\23.0\bin64;C:\Program Files\Microsoft\jdk-11.0.10.9-hotspot\bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0\;C:\Program Files\SafeNet\Authentication\SAC\x64;C:\Program Files\SafeNet\Authentication\SAC\x32;C:\Program Files\Microsoft VS Code\bin;C:\Program Files\dotnet\;C:\Program Files (x86)\IVI Foundation\VISA\WinNT\Bin\;C:\Program Files\IVI Foundation\VISA\Win64\Bin\;C:\Program Files (x86)\IVI Foundation\VISA\WinNT\Bin;C:\ProgramData\chocolatey\bin;C:\Program Files\VitalElement\AvalonBuild\;C:\JediTools\Jsf;C:\JediTools\Jsf\PlugIns;C:\JediTools\Common;C:\Program Files (x86)\Microsoft SQL Server\80\Tools\Binn\;C:\Program Files\Pandoc\;C:\Program Files (x86)\GitExtensions\;C:\Program Files\gs\gs10.00.0\bin;C:\Program Files\IVI Foundation\IVI\Bin\;C:\Program Files (x86)\NTP\bin;C:\Program Files (x86)\Yarn\bin\;C:\Program Files (x86)\DXR_OEM\DAB3;C:\Program Files (x86)\nodejs\;C:\Program Files (x86)\National Instruments\Shared\LabVIEW CLI;C:\Program Files\Microsoft SQL Server\150\Tools\Binn\;C:\Program Files\CMake\bin;C:\Program Files\Git\cmd;C:\Users\108003658\.cargo\bin;C:\Users\108003658\AppData\Local\Microsoft\WindowsApps;C:\Users\108003658\.dotnet\tools;C:\Users\108003658\AppData\Local\GitHubDesktop\bin;C:\Users\108003658\AppData\Local\Yarn\bin;C:\tools\ghc-9.8.1\bin;C:\MinGW64\bin;C:\Program Files (x86)\Nmap;C:\Program Files (x86)\Common Files\Hilscher GmbH\TLRDecode;C:\Users\108003658\.cargo\bin;C:\Users\108003658\AppData\Local\Microsoft\WindowsApps;C:\Users\108003658\.dotnet\tools;C:\Users\108003658\AppData\Local\GitHubDesktop\bin;C:\Users\108003658\AppData\Local\Yarn\bin;C:\tools\ghc-9.8.1\bin;C:\MinGW64\bin;C:\Program Files (x86)\Nmap;C:\Program Files (x86)\Common Files\Hilscher GmbH\TLRDecode;C:\BRAutomation\AS412\bin-en\4.12;C:\BRAutomation\AS412\bin-en\4.11;C:\BRAutomation\AS412\bin-en\4.10;C:\BRAutomation\AS412\bin-en\4.9;C:\BRAutomation\AS412\bin-en\4.8;C:\BRAutomation\AS412\bin-en\4.7;C:\BRAutomation\AS412\bin-en\4.6;C:\BRAutomation\AS412\bin-en\4.5;C:\BRAutomation\AS412\bin-en\4.4;C:\BRAutomation\AS412\bin-en\4.3;C:\BRAutomation\AS412\bin-en\4.2;C:\BRAutomation\AS412\bin-en\4.1;C:\BRAutomation\AS412\bin-en\4.0;C:\BRAutomation\AS412\bin-en
export AS_BUILD_MODE := BuildAndTransfer
export AS_VERSION := 4.12.5.95 SP
export AS_WORKINGVERSION := 4.12
export AS_COMPANY_NAME := Baker Hughes
export AS_USER_NAME := 108003658
export AS_PATH := C:/BRAutomation/AS412
export AS_BIN_PATH := C:/BRAutomation/AS412/bin-en
export AS_PROJECT_PATH := C:/projects/X20CP1586AS4SHA256rdtsc
export AS_PROJECT_NAME := X20CP1586AS4SHA256
export AS_SYSTEM_PATH := C:/BRAutomation/AS/System
export AS_VC_PATH := C:/BRAutomation/AS412/AS/VC
export AS_TEMP_PATH := C:/projects/X20CP1586AS4SHA256rdtsc/Temp
export AS_CONFIGURATION := Config1
export AS_BINARIES_PATH := C:/projects/X20CP1586AS4SHA256rdtsc/Binaries
export AS_GNU_INST_PATH := C:/BRAutomation/AS412/AS/GnuInst/V4.1.2
export AS_GNU_BIN_PATH := C:/BRAutomation/AS412/AS/GnuInst/V4.1.2/4.9/bin
export AS_GNU_INST_PATH_SUB_MAKE := C:/BRAutomation/AS412/AS/GnuInst/V4.1.2
export AS_GNU_BIN_PATH_SUB_MAKE := C:/BRAutomation/AS412/AS/GnuInst/V4.1.2/4.9/bin
export AS_INSTALL_PATH := C:/BRAutomation/AS412
export WIN32_AS_PATH := "C:\BRAutomation\AS412"
export WIN32_AS_BIN_PATH := "C:\BRAutomation\AS412\bin-en"
export WIN32_AS_PROJECT_PATH := "C:\projects\X20CP1586AS4SHA256rdtsc"
export WIN32_AS_SYSTEM_PATH := "C:\BRAutomation\AS\System"
export WIN32_AS_VC_PATH := "C:\BRAutomation\AS412\AS\VC"
export WIN32_AS_TEMP_PATH := "C:\projects\X20CP1586AS4SHA256rdtsc\Temp"
export WIN32_AS_BINARIES_PATH := "C:\projects\X20CP1586AS4SHA256rdtsc\Binaries"
export WIN32_AS_GNU_INST_PATH := "C:\BRAutomation\AS412\AS\GnuInst\V4.1.2"
export WIN32_AS_GNU_BIN_PATH := "C:\BRAutomation\AS412\AS\GnuInst\V4.1.2\bin"
export WIN32_AS_INSTALL_PATH := "C:\BRAutomation\AS412"

.suffixes:

ProjectMakeFile:

	@'$(AS_BIN_PATH)/4.9/BR.AS.AnalyseProject.exe' '$(AS_PROJECT_PATH)/X20CP1586AS4SHA256.apj' -t '$(AS_TEMP_PATH)' -c '$(AS_CONFIGURATION)' -o '$(AS_BINARIES_PATH)'   -sfas -buildMode 'BuildAndTransfer'   

