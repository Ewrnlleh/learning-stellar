# GitHub'a Alt Klasör Olarak Push Etme Rehberi

## Adım 1: Ana Repo'yu Klonlayın
```bash
cd c:\Users\Can\Desktop
git clone https://github.com/[USERNAME]/[REPO_NAME].git
cd [REPO_NAME]
```

## Adım 2: Freezable Token Klasörü Oluşturun
```bash
mkdir freezable-token-contract
cd freezable-token-contract
```

## Adım 3: Proje Dosyalarını Kopyalayın
```bash
# PowerShell'de:
Copy-Item -Recurse -Path "c:\Users\Can\Desktop\freezable_token_contract\*" -Destination "."
```

## Adım 4: Git'e Ekleyin ve Push Edin
```bash
git add freezable-token-contract/
git commit -m "Add freezable token contract with freeze functionality

Features:
- freeze_account() and unfreeze_account() functions (admin only)  
- Enhanced transfer() and transfer_from() with freeze checks
- Comprehensive test suite and documentation
- Ready for Stellar testnet deployment"

git push origin main
```

## Alternatif: Mevcut Klasörü Taşıma
Eğer repo zaten varsa:

```bash
# Ana repo'da
mkdir freezable-token-contract
cd freezable-token-contract

# Dosyaları kopyala
Copy-Item -Recurse "c:\Users\Can\Desktop\freezable_token_contract\*" .

# Gereksiz dosyaları temizle
Remove-Item target -Recurse -Force
Remove-Item Cargo.lock

git add .
git commit -m "Add freezable token smart contract"
git push
```

## Önerilen Klasör Yapısı:
```
your-repo/
├── README.md
├── other-projects/
└── freezable-token-contract/
    ├── src/
    │   ├── contract.rs
    │   ├── lib.rs
    │   └── test.rs
    ├── Cargo.toml
    ├── README.md
    ├── DEPLOYMENT_SUCCESS.md
    └── build.bat
```

## .gitignore Eklentileri
Freezable token klasörü için `.gitignore`'a ekleyin:

```gitignore
# Rust/Cargo
freezable-token-contract/target/
freezable-token-contract/Cargo.lock

# Soroban
*.wasm
.soroban/
```
