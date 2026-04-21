@echo off
setlocal EnableExtensions EnableDelayedExpansion

reg add HKCU\Console /v VirtualTerminalLevel /t REG_DWORD /d 1 /f >nul 2>&1

chcp 65001 >nul 2>&1

set "TOR_VERSION=14.5.6"
set "ARCH=x86_64"
set "BUNDLE_NAME=tor-expert-bundle-windows-%ARCH%-%TOR_VERSION%.tar.gz"
set "URL=https://archive.torproject.org/tor-package-archive/torbrowser/%TOR_VERSION%/%BUNDLE_NAME%"

set "REPO_ROOT=%~dp0"
set "DOWNLOAD_DIR=%REPO_ROOT%build-tmp"
set "TARGET_DIR=%REPO_ROOT%resources\tor"
set "ARCHIVE=%DOWNLOAD_DIR%\%BUNDLE_NAME%"

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
echo    [90m  Tor runtime setup - fetches dependencies from the Tor Project[0m
echo    [90m  ------------------------------------------------------------[0m
echo.
echo.

echo   [96m[info][0m  Target folder : [97m%TARGET_DIR%[0m
echo   [96m[info][0m  Tor version   : [97m%TOR_VERSION% (%ARCH%)[0m
echo.

if not exist "%DOWNLOAD_DIR%" mkdir "%DOWNLOAD_DIR%"
if not exist "%TARGET_DIR%" mkdir "%TARGET_DIR%"

echo   [96m[1/4][0m Downloading bundle...
curl -L --fail --progress-bar -o "%ARCHIVE%" "%URL%"
if errorlevel 1 (
  echo   [91m[error][0m Download failed. Check your internet connection.
  goto :fail
)
echo.

echo   [96m[2/4][0m Extracting archive...
tar -xzf "%ARCHIVE%" -C "%DOWNLOAD_DIR%"
if errorlevel 1 (
  echo   [91m[error][0m Extraction failed. Is tar.exe available?
  goto :fail
)
if not exist "%DOWNLOAD_DIR%\tor" (
  echo   [91m[error][0m Expected 'tor\' folder not found inside the archive.
  goto :fail
)
echo.

echo   [96m[3/4][0m Copying Tor binaries into resources\tor ...
xcopy /E /I /Y /Q "%DOWNLOAD_DIR%\tor\*" "%TARGET_DIR%" >nul
if errorlevel 1 (
  echo   [91m[error][0m Copy of Tor binaries failed.
  goto :fail
)
if exist "%DOWNLOAD_DIR%\data" (
  copy /Y "%DOWNLOAD_DIR%\data\geoip*" "%TARGET_DIR%\" >nul 2>&1
)
echo.

echo   [96m[4/4][0m Cleaning up...
rmdir /S /Q "%DOWNLOAD_DIR%"
echo.

echo.
echo   [92m[ok][0m All dependencies installed successfully.
echo.
echo   [90mNext step:[0m
echo     [93mcargo build --release[0m
echo.
echo   [90mThe resulting .exe at target\release\onionymous.exe is fully[0m
echo   [90mportable. You can move it to your Desktop or anywhere else[0m
echo   [90mand it will still work on its own.[0m
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
