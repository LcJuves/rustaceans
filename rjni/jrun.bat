@echo off
:: Created at 2021/7/26 17:09
:: @author Liangcheng Juves

Set ExecDir=%cd%
Set BaseDir=%~dp0
Set RunType=release

Set JavaHomeArray="%JAVA_HOME%"
Set JavaHomeArray=%JavaHomeArray%;"C:\Program Files\Java\jdk1.6.0"

for %%j in (%JavaHomeArray%) do (
:Continue
    if exist %%j (
        call:Once %%j
    )
)
cd "%ExecDir%"
goto:eof

::::::::::::::::::::::::::::::::::::
::::::::::::::::::::::::::::::::::::
::::::::::::::::::::::::::::::::::::

:Once

Set JavaHome="%~1"

Set JavacPath="%JavaHome%\bin\javac"
Set JavaPath="%JavaHome%\bin\java"

@echo.
"%JavaPath%" -version
echo ============================================================
echo ============================================================
@echo.

cd "%BaseDir%" && cargo build --%RunType%
cd "%BaseDir%\jcalls" && "%JavacPath%" Main.java
cd "%BaseDir%\jcalls" && "%JavaPath%" -Djava.library.path="%BaseDir%\target\%RunType%" Main

@echo.
echo ///////////////////////////////////////////////////////////////
echo ///////////////////////////////////////////////////////////////
echo ///////////////////////////////////////////////////////////////

goto:Continue