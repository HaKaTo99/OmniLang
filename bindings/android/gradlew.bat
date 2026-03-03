@echo off
set DIR=%~dp0
"%JAVA_HOME%\bin\java" -jar "%DIR%gradle\wrapper\gradle-wrapper.jar" %*
