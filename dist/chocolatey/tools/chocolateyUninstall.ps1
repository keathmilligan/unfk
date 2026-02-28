$ErrorActionPreference = 'Stop'

$packageName = 'unfk'
$softwareName = 'unfk*'

[array]$key = Get-UninstallRegistryKey -SoftwareName $softwareName

if ($key.Count -eq 1) {
  $key | ForEach-Object {
    $packageArgs = @{
      packageName    = $packageName
      fileType       = 'msi'
      silentArgs     = "$($_.PSChildName) /qn /norestart"
      validExitCodes = @(0, 1605, 1614, 1641, 3010)
      file           = ''
    }
    Uninstall-ChocolateyPackage @packageArgs
  }
} elseif ($key.Count -gt 1) {
  Write-Warning "$($key.Count) matches found! This package will need manual cleanup."
  $key | ForEach-Object { Write-Warning "- $($_.DisplayName)" }
}
