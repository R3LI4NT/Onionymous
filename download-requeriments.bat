@echo off
setlocal EnableExtensions EnableDelayedExpansion

REM ==========================================================================
REM  Onionymous v1.1 - download-requeriments.bat
REM ==========================================================================
REM  Fetches:
REM    1. Tor Expert Bundle (tor.exe + geoip + pluggable transports)
REM    2. Wintun.dll (TUN driver for Windows by WireGuard)
REM    3. sing-box.exe (TUN-to-SOCKS bridging engine)
REM ==========================================================================

reg add HKCU\Console /v VirtualTerminalLevel /t REG_DWORD /d 1 /f >nul 2>&1
chcp 65001 >nul 2>&1

set "TOR_VERSION=15.0.9"
set "ARCH=x86_64"
set "BUNDLE_NAME=tor-expert-bundle-windows-%ARCH%-%TOR_VERSION%.tar.gz"
set "URL_TOR=https://archive.torproject.org/tor-package-archive/torbrowser/%TOR_VERSION%/%BUNDLE_NAME%"

set "WINTUN_VERSION=0.14.1"
set "WINTUN_ZIP=wintun-%WINTUN_VERSION%.zip"
set "URL_WINTUN=https://www.wintun.net/builds/%WINTUN_ZIP%"

set "SINGBOX_VERSION=1.12.24"
set "SINGBOX_ZIP=sing-box-%SINGBOX_VERSION%-windows-amd64.zip"
set "SINGBOX_DIR=sing-box-%SINGBOX_VERSION%-windows-amd64"
set "URL_SINGBOX=https://github.com/SagerNet/sing-box/releases/download/v%SINGBOX_VERSION%/%SINGBOX_ZIP%"

set "REPO_ROOT=%~dp0"
set "DOWNLOAD_DIR=%REPO_ROOT%build-tmp"
set "TOR_TARGET=%REPO_ROOT%resources\tor"
set "WINTUN_TARGET=%REPO_ROOT%resources\wintun"
set "SINGBOX_TARGET=%REPO_ROOT%resources\singbox"
set "ARCHIVE_TOR=%DOWNLOAD_DIR%\%BUNDLE_NAME%"
set "ARCHIVE_WINTUN=%DOWNLOAD_DIR%\%WINTUN_ZIP%"
set "ARCHIVE_SINGBOX=%DOWNLOAD_DIR%\%SINGBOX_ZIP%"

cls
echo.
echo    [38;2;138;92;255m   ____        _                                                   [0m
echo    [38;2;138;92;255m  / __ \      (_)                                                  [0m
echo    [38;2;138;92;255m ^| ^|  ^| ^|_ __  _  ___  _ __  _   _ _ __ ___   ___  _   _ ___       [0m
echo    [38;2;138;92;255m ^| ^|  ^| ^| _ \^| ^|/ _ \^| _ \^| ^| ^| ^| _ ` _ \ / _ \^| ^| ^| / __^|      {X}
echo    [38;2;138;92;255m ^| ^|__^| ^| ^| ^| ^| ^| ^(_^) ^| ^| ^| ^| ^|_^| ^| ^| ^| ^| ^| ^| ^(_^) ^| ^|_^| \__ \      [0m
echo    [38;2;138;92;255m  \____/^|_^| ^|_^|_^|\___/^|_^| ^|_^|\__, ^|_^| ^|_^| ^|_^|\___/ \__,_^|___/      [0m
echo    [38;2;138;92;255m                               __/ ^|                               [0m
echo    [38;2;138;92;255m                              ^|___/                                [0m
echo.
echo    [90m  v1.1 Tor + Wintun + sing-box dependencies[0m
echo    [90m  ------------------------------------------[0m
echo.

echo   [96m[info][0m  Tor      : [97m%TOR_VERSION%[0m
echo   [96m[info][0m  Wintun   : [97m%WINTUN_VERSION%[0m
echo   [96m[info][0m  sing-box : [97m%SINGBOX_VERSION%[0m
echo.

if not exist "%DOWNLOAD_DIR%" mkdir "%DOWNLOAD_DIR%"
if not exist "%TOR_TARGET%" mkdir "%TOR_TARGET%"
if not exist "%WINTUN_TARGET%" mkdir "%WINTUN_TARGET%"
if not exist "%SINGBOX_TARGET%" mkdir "%SINGBOX_TARGET%"

echo   [96m[1/8][0m Downloading Tor Expert Bundle...
curl -L --fail --progress-bar -o "%ARCHIVE_TOR%" "%URL_TOR%"
if errorlevel 1 (
  echo   [91m[error][0m Tor download failed.
  goto :fail
)
echo.

echo   [96m[2/8][0m Extracting Tor bundle...
tar -xzf "%ARCHIVE_TOR%" -C "%DOWNLOAD_DIR%"
if errorlevel 1 (
  echo   [91m[error][0m Tor extraction failed.
  goto :fail
)
echo.

echo   [96m[3/8][0m Copying Tor files into resources\tor ...
xcopy /E /I /Y /Q "%DOWNLOAD_DIR%\tor\*" "%TOR_TARGET%" >nul
if errorlevel 1 (
  echo   [91m[error][0m Tor copy failed.
  goto :fail
)
if exist "%DOWNLOAD_DIR%\data" (
  copy /Y "%DOWNLOAD_DIR%\data\geoip*" "%TOR_TARGET%\" >nul 2>&1
)
echo.

echo   [96m[4/8][0m Downloading Wintun driver...
curl -L --fail --progress-bar -o "%ARCHIVE_WINTUN%" "%URL_WINTUN%"
if errorlevel 1 (
  echo   [91m[error][0m Wintun download failed.
  goto :fail
)
echo.

echo   [96m[5/8][0m Extracting Wintun...
tar -xf "%ARCHIVE_WINTUN%" -C "%DOWNLOAD_DIR%"
if errorlevel 1 (
  echo   [91m[error][0m Wintun extraction failed.
  goto :fail
)
if exist "%DOWNLOAD_DIR%\wintun\bin\amd64\wintun.dll" (
  copy /Y "%DOWNLOAD_DIR%\wintun\bin\amd64\wintun.dll" "%WINTUN_TARGET%\wintun.dll" >nul
) else (
  echo   [91m[error][0m wintun.dll not found inside the zip.
  goto :fail
)
echo.

echo   [96m[6/8][0m Downloading sing-box...
curl -L --fail --progress-bar -o "%ARCHIVE_SINGBOX%" "%URL_SINGBOX%"
if errorlevel 1 (
  echo   [91m[error][0m sing-box download failed.
  goto :fail
)
echo.

echo   [96m[7/8][0m Extracting sing-box...
tar -xf "%ARCHIVE_SINGBOX%" -C "%DOWNLOAD_DIR%"
if errorlevel 1 (
  echo   [91m[error][0m sing-box extraction failed.
  goto :fail
)
if exist "%DOWNLOAD_DIR%\%SINGBOX_DIR%\sing-box.exe" (
  copy /Y "%DOWNLOAD_DIR%\%SINGBOX_DIR%\sing-box.exe" "%SINGBOX_TARGET%\sing-box.exe" >nul
) else (
  echo   [91m[error][0m sing-box.exe not found inside the zip.
  goto :fail
)
echo.

echo   [96m[8/8][0m Cleaning up...
rmdir /S /Q "%DOWNLOAD_DIR%"
echo.

echo   [92m[ok][0m All dependencies installed successfully.
echo.
echo   [90mFiles placed:[0m
echo     [97mresources\tor\          (Tor binary + DLLs + PTs)[0m
echo     [97mresources\wintun\       (wintun.dll)[0m
echo     [97mresources\singbox\      (sing-box.exe)[0m
echo.
echo   [90mNext step:[0m
echo     [93mcargo build --release[0m
echo.
echo   [90mRun as ADMINISTRATOR for TUN/VPN mode to work.[0m
echo.
endlocal
pause
exit /b 0

:fail
echo.
echo   [91m[fatal][0m Setup aborted. See messages above.
if exist "%DOWNLOAD_DIR%" rmdir /S /Q "%DOWNLOAD_DIR%" 2>nul
echo.
endlocal
pause
exit /b 1
