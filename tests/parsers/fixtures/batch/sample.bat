@echo off
:: Module documentation for Batch testing
:: This is also a module comment

REM Regular comment - ignored
@REM Also ignored
; Also ignored

set CONFIG_DIR=%USERPROFILE%\.vibe-analyzer
set MAX_SIZE=1024
set TIMEOUT=30

:: Function to add two numbers
:add
set /a result=%1 + %2
echo %result%
goto :eof

:: Function to multiply two numbers
:multiply
set /a result=%1 * %2
echo %result%
goto :eof

:: Function to check if file exists
:file_exists
if exist "%1" (
    exit /b 0
) else (
    exit /b 1
)

REM Function with multiple lines - ignored
:process_data
setlocal enabledelayedexpansion
set input=%1
set output=

for /l %%i in (1,1,10) do (
    set output=!output! %%i
)

echo !output!
endlocal
goto :eof

call utils.bat
call config.bat
