@echo off
echo GitHub'a Freezable Token Contract Push Etme
echo =============================================

REM Repo bilgileri:
set GITHUB_USERNAME=Ewrnlleh
set REPO_NAME=learning-stellar
set SUBFOLDER_NAME=freezable-token-contract

echo.
echo 1. Ana repo'yu klonluyorum...
cd c:\Users\Can\Desktop
git clone https://github.com/%GITHUB_USERNAME%/%REPO_NAME%.git
cd %REPO_NAME%

echo.
echo 2. Alt klasor olusturuyorum...
mkdir %SUBFOLDER_NAME%
cd %SUBFOLDER_NAME%

echo.
echo 3. Proje dosyalarini kopyaliyorum...
xcopy /E /I "c:\Users\Can\Desktop\freezable_token_contract\*" .

echo.
echo 4. Git'e ekliyorum...
cd ..
git add %SUBFOLDER_NAME%/
git commit -m "Add freezable token contract with freeze functionality

✅ Final Project: Stellar Smart Contract with Freeze Functionality

Features Implemented:
- freeze_account() function (admin only)
- unfreeze_account() function (admin only) 
- Enhanced transfer() with freeze protection
- Enhanced transfer_from() with freeze protection
- Comprehensive test suite and documentation
- Ready for Stellar testnet deployment

This contract extends standard token functionality to allow administrators
to freeze/unfreeze specific accounts, preventing frozen accounts from 
transferring tokens while still allowing them to receive tokens.

Project completed: June 26, 2025"

echo.
echo 5. GitHub'a push ediyorum...
git push origin main

echo.
echo ✅ Basariyla tamamlandi!
echo Repo linki: https://github.com/%GITHUB_USERNAME%/%REPO_NAME%/tree/main/%SUBFOLDER_NAME%

pause
