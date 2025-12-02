@echo off
mkdir inputs

for /L %%i in (1,1,25) do (
    setlocal enabledelayedexpansion
    if %%i LSS 10 (
        set filename=day0%%i.txt
    ) else (
        set filename=day%%i.txt
    )
    echo. > inputs\!filename!
    endlocal
)