$ErrorActionPreference = 'Stop'

$packageName = 'unfk'
$url = 'https://github.com/keathmilligan/unfk/releases/download/v1.1.0/unfk-1.1.0-x86_64.msi'

$packageArgs = @{
  packageName    = $packageName
  fileType       = 'msi'
  url64bit       = $url
  softwareName   = 'unfk*'
  silentArgs     = '/qn /norestart'
  validExitCodes = @(0, 3010, 1641)
}

Install-ChocolateyPackage @packageArgs
