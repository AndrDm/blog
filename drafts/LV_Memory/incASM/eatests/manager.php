<?php /* manager.php is proxy script for €ASM test manager testman.php.
       It is invoked from eatests/t????.htm by the button [Manage test]. */
function GetTestId($ReqName) // Returns submitted test, e.g. 't1234' or ''.
{$test=@$_REQUEST[$ReqName]; // Accepts only four decimal digits, optionally prefixed with 't'.
$testNr='';
for ($i=0;$i<strlen($test);$i++) if (($test{$i}<='9') && ($test{$i}>='0')) $testNr.=$test{$i};
if ($testNr<=9999 && $testNr>0) return 't'.substr("000$testNr",-4);
else return 'index';
} // endFn GetTestId
$test=GetTestId('test');
$SLASH=PHP_OS=="Linux"?"/":"\\";
if (file_exists(strtr(substr($_SERVER['SCRIPT_FILENAME'],0,-11),"/",$SLASH)."testman.php"))
  header("Location: testman.php?test=$test");
else echo "<!doctype html><html><head>
<meta http-equiv='Content-Type' content='text/html; charset=utf-8'/>
<meta http-equiv='Content-Language' content='en'/>
<meta http-equiv='refresh' content='5; url=$test.htm'>
<link rel='stylesheet' href='../euroasm.css' type='text/css'/>
<link rel='shortcut icon' href='../favicon.ico'/>
<title>Test Manager</title>
</head>
<body class='EATESTS' id='top'>
<dl class='STATUS'><del><dt>Test management is not available on this site.</dt>
<dd><br/>You should install EuroAssembler on your localhost webserver
and unzip the script <q>eatests/testman.php</q> from archive <q>generate.php</q>
if you want to edit and execute the tests from browser.
<br/><br/>Returning back to <a href='$test.htm'>$test</a> ...</dd></del></dl>
</body></html>";
?>

