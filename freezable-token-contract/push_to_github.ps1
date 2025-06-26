# GitHub'a Freezable Token Contract Push Etme
Write-Host "GitHub'a Freezable Token Contract Push Etme" -ForegroundColor Green
Write-Host "=============================================" -ForegroundColor Green

# Repo bilgileri
$GITHUB_USERNAME = "Ewrnlleh"
$REPO_NAME = "learning-stellar"
$SUBFOLDER_NAME = "freezable-token-contract"

Write-Host ""
Write-Host "1. Ana repo'yu klonlıyorum..." -ForegroundColor Yellow
Set-Location "c:\Users\Can\Desktop"
git clone "https://github.com/$GITHUB_USERNAME/$REPO_NAME.git"
Set-Location $REPO_NAME

Write-Host ""
Write-Host "2. Alt klasör oluşturuyorum..." -ForegroundColor Yellow
New-Item -ItemType Directory -Name $SUBFOLDER_NAME -Force
Set-Location $SUBFOLDER_NAME

Write-Host ""
Write-Host "3. Proje dosyalarını kopyalıyorum..." -ForegroundColor Yellow
Copy-Item -Recurse -Path "c:\Users\Can\Desktop\freezable_token_contract\*" -Destination "." -Exclude @("target", "Cargo.lock", ".git")

Write-Host ""
Write-Host "4. Git'e ekliyorum..." -ForegroundColor Yellow
Set-Location ".."
git add "$SUBFOLDER_NAME/"
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

Write-Host ""
Write-Host "5. GitHub'a push ediyorum..." -ForegroundColor Yellow
git push origin main

Write-Host ""
Write-Host "✅ Başarıyla tamamlandı!" -ForegroundColor Green
Write-Host "Repo linki: https://github.com/$GITHUB_USERNAME/$REPO_NAME/tree/main/$SUBFOLDER_NAME" -ForegroundColor Cyan

Read-Host "Devam etmek için Enter'a basın"
