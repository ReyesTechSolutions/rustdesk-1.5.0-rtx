# Guía de compilación — Cliente REYES TECH SOLUTIONS (Windows x64)

Versiones tomadas de la CI oficial del repo (`.github/workflows/flutter-build.yml` y `bridge.yml`) para que el build local sea idéntico al oficial.

> **Tiempo total estimado:** 2–4 h la primera vez (la mayoría es vcpkg + cargo).
> **Espacio en disco:** ~40 GB libres.

---

## Paso 0 — Requisitos

- Windows 10/11 x64 con conexión a internet
- ~40 GB libres en disco
- Git ya instalado ✓ (tienes 2.55)

---

## Paso 1 — Visual Studio (compilador C++ requerido por Rust)

1. Descarga **Visual Studio 2022 Community** (gratis): https://visualstudio.microsoft.com/es/downloads/
2. En el instalador, marca la carga de trabajo:
   - **"Desarrollo para el escritorio con C++"**
3. Instala (~8 GB).

Verifica después abriendo "Developer PowerShell for VS 2022":

```powershell
where.exe cl.exe
```

---

## Paso 2 — Rust

En PowerShell normal:

```powershell
winget install Rustlang.Rustup --accept-source-agreements --accept-package-agreements
```

Cierra y reabre PowerShell, luego:

```powershell
rustup default 1.75.0
rustup component add rustfmt clippy
rustc --version   # debe decir: rustc 1.75.0
```

> ¿Por qué 1.75.0? Es la `RUST_VERSION` fijada en la CI del proyecto. Versiones
> más nuevas también suelen compilar, pero 1.75 es la garantizada.

---

## Paso 3 — Flutter 3.24.5 (versión exacta de la CI)

```powershell
# Carpeta recomendada (evita rutas con espacios)
mkdir C:\flutter -Force
# Descarga el zip oficial de la versión
Invoke-WebRequest -Uri "https://storage.googleapis.com/flutter_infra_release/releases/stable/windows/flutter_windows_3.24.5-stable.zip" -OutFile "C:\flutter.zip"
Expand-Archive -Path "C:\flutter.zip" -DestinationPath "C:\"
# Añadir al PATH de la sesión
$env:PATH = "C:\flutter\bin;$env:PATH"
```

Para hacerlo permanente:

```powershell
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\flutter\bin", "User")
```

Verifica:

```powershell
flutter --version    # Flutter 3.24.5
flutter doctor
```

> **Importante (Windows x64):** la CI sustituye el engine de Flutter por uno
> custom de RustDesk. Ejecuta estos dos pasos tras instalar Flutter:
>
> ```powershell
> flutter precache --windows
> Invoke-WebRequest -Uri "https://github.com/rustdesk/engine/releases/download/main/windows-x64-release.zip" -OutFile "C:\win-engine.zip"
> Expand-Archive -Path "C:\win-engine.zip" -DestinationPath "C:\win-engine"
> $engineDir = "C:\flutter\bin\cache\artifacts\engine\windows-x64-release"
> Remove-Item -Recurse -Force $engineDir -ErrorAction SilentlyContinue
> Copy-Item -Recurse "C:\win-engine" $engineDir
> ```
>
> (Si `flutter doctor` o el build fallan raro con el engine stock, es por esto.)

---

## Paso 4 — vcpkg 2026.07.29 (commit exacto de la CI)

```powershell
git clone https://github.com/microsoft/vcpkg C:\vcpkg
cd C:\vcpkg
git checkout 9e593bb18ea69cc5095e012465dcd675a822ed0d
.\bootstrap-vcpkg.bat -disableMetrics
```

Variables de entorno permanentes:

```powershell
[Environment]::SetEnvironmentVariable("VCPKG_ROOT", "C:\vcpkg", "User")
```

> La primera vez que compiles, vcpkg descargará y compilará libvpx, libyuv,
> opus, aom, ffmpeg, libsodium… (~1–2 h). Es normal. Con `VCPKG_ROOT` definido
> y el `vcpkg.json` del repo, `cargo build` los instala solo.

Instalación manual (opcional, para adelantar):

```powershell
cd C:\vcpkg
.\vcpkg install --triplet x64-windows-static --x-install-root="C:\vcpkg\installed"
```

---

## Paso 5 — Python 3 + herramientas menores

```powershell
winget install Python.Python.3.12 --accept-source-agreements --accept-package-agreements
```

(Se usa para `build.py`, `res/msi/preprocess.py` y scripts auxiliares.)

---

## Paso 6 — Clonar el código con submódulos

Tu carpeta de trabajo personal (donde mantuviste la personalización) **no es un
repositorio git** y le falta `libs/hbb_common` (submodule) y los archivos del
puente generados (`src/bridge_generated.rs`, `flutter/lib/generated_bridge.dart`).
Lo más fiable es clonar limpio y aplicar tu personalización encima:

```powershell
cd C:\
git clone --recurse-submodules https://github.com/rustdesk/rustdesk C:\rustdesk
```

---

## Paso 7 — Aplicar tu personalización

Copia `aplicar-personalizacion.ps1` (está en la raíz de tu carpeta actual) y
ejecútalo apuntando al clon:

```powershell
powershell -ExecutionPolicy Bypass -File "C:\ruta\a\aplicar-personalizacion.ps1" -Destino C:\rustdesk
```

Esto copia:
- Nombre de app, textos, tema de colores, iconos (todas las plataformas)
- Splash Android, logos del login, Runner.rc, manifests, MSI/preprocess.py

---

## Paso 8 — Generar el puente Rust↔Dart (flutter_rust_bridge)

El repo no trae los archivos generados; la CI los produce con la versión exacta
`flutter_rust_bridge_codegen 1.80.1`. En PowerShell (con Rust y Flutter ya en PATH):

```powershell
cargo install cargo-expand --version 1.0.95 --locked
cargo install flutter_rust_bridge_codegen --version 1.80.1 --features "uuid" --locked

cd C:\rustdesk\flutter
flutter pub get
cd ..

# generar el puente (necesita cl.exe en PATH → usa el Developer PowerShell de VS)
~\.cargo\bin\flutter_rust_bridge_codegen.exe `
    --rust-input ./src/flutter_ffi.rs `
    --dart-output ./flutter/lib/generated_bridge.dart `
    --c-output ./flutter/macos/Runner/bridge_generated.h
```

Deben existir después: `src/bridge_generated.rs`, `src/bridge_generated.io.rs`,
`flutter/lib/generated_bridge.dart`, `flutter/lib/generated_bridge.freezed.dart`.

> El `pubspec.lock` del repo a veces necesita regenerarse:
> si `flutter pub get` falla por `extended_text`, edita `flutter/pubspec.yaml`
> y baja `extended_text: 14.0.0` → `13.0.0` (igual que hace la CI para Flutter 3.22).

---

## Paso 9 — Compilar el exe con tu marca

En el **Developer PowerShell for VS 2022** (para tener `cl.exe`):

```powershell
cd C:\rustdesk
$env:VCPKG_ROOT = "C:\vcpkg"
$env:PATH = "C:\flutter\bin;$env:PATH"

python build.py --flutter --hwcodec --vram --portable --skip-portable-pack
```

Resultado:
- **Rust:** `target/release/librustdesk.dll` (~20–40 min)
- **Flutter:** `flutter\build\windows\x64\runner\Release\` con `rustdesk.exe` dentro
- El exe lleva tu icono, tus metadatos (Runner.rc) y, al arrancar, el nombre
  "Reyes Tech Solutions" (viene de `src/common.rs`).

Prueba rápida:

```powershell
.\flutter\build\windows\x64\runner\Release\rustdesk.exe
```

Debe mostrar tu marca en la ventana, tu tema de colores y funcionar contra los
servidores públicos sin configurar nada.

---

## Paso 10 — Instalador MSI con tu marca

```powershell
cd C:\rustdesk\res\msi
# Copia la carpeta Release a un dist con el nombre que el MSI espera
# El MSI busca <app-name>.exe → "Reyes Tech Solutions.exe" con espacios se rompe,
# por eso el flujo recomendado es usar el nombre corto:
python preprocess.py --arp -d ..\..\flutter\build\windows\x64\runner\Release `
    --app-name "Reyes Tech Solutions" -m "Reyes Tech Solutions"

nuget restore msi.sln
msbuild msi.sln -p:Configuration=Release -p:Platform=x64 /p:TargetVersion=Windows10
```

> `nuget` y `msbuild` vienen con Visual Studio / VS Build Tools.
> El MSI resultante está en `Package\bin\x64\Release\en-us\Package.msi`.
>
> Nota: `init_global_vars` ejecuta `dist_app --version` y `--build-date`; el
> exe del dist se renombra a `Reyes Tech Solutions.exe` automáticamente por el
> script solo si el archivo existe con ese nombre exacto. Si falla, renómbralo
> a mano antes de preprocess.py.

Alternativa más simple — **instalador portable self-extracting** (el que usa la
CI por defecto, no requiere WiX):

```powershell
cd C:\rustdesk\libs\portable
pip3 install -r requirements.txt
python .\generate.py -f ..\..\flutter\build\windows\x64\runner\Release\ -o . -e ..\..\flutter\build\windows\x64\runner\Release\rustdesk.exe
# Resultado: target/release/rustdesk-portable-packer.exe → renómbralo a
# "ReyesTech-Setup-1.5.0-x64.exe" y ya es distribuible
```

---

## Paso 11 — Firma Authenticode (quitar el aviso de Windows SmartScreen)

Los instaladores sin firma disparan "Windows protegió tu equipo" (SmartScreen) al
descargarlos/ejecutarlos. El flujo ya está preparado y probado en esta máquina.

**Script:** `firmar-instaladores.ps1` (raíz del proyecto). Requiere `signtool.exe`
(ya presente con el Windows SDK de Visual Studio).

```powershell
# Firmar con el PFX del certificado comprado (recomendado):
.\firmar-instaladores.ps1 -Modo PFX -Pfx "C:\certs\reyes.pfx"

# Con token USB / certificado en el store (usa la huella SHA1):
.\firmar-instaladores.ps1 -Modo STORE -Huella <thumbprint>

# Solo comprobar firmas existentes:
.\firmar-instaladores.ps1 -SoloVerificar
```

El script firma con **SHA256**, aplica **timestamp RFC3161** (DigiCert) — la firma
sigue siendo válida aunque el certificado caduque — y verifica con
`signtool verify /pa /all`.

**Opciones de certificado (2026):**

| Opción | Precio aprox. | Efecto SmartScreen |
|---|---|---|
| **OV Code Signing** (Sectigo, SSL.com, Certum Open Source ~$89/año) | $70–120/año | Firma válida; reputación se construye con descargas (días/semanas de aviso decreciente) |
| **EV Code Signing** (Sectigo/DigiCert; requiere entidad validada) | $250–400/año | Reputación inmediata desde la primera firma (históricamente; verificar política vigente) |
| **Azure Trusted Signing** (recomendado) | ~$9.99/mes | Reputación gestionada por Microsoft; integración `signtool /dlib`; identidad validada una vez |

> Nota 2024+: los certificados OV/EV se emiten solo en token USB físico o
> HSM (ya no se entregan PFX exportables) → usa `-Modo STORE`. Con Azure
> Trusted Signing no hay token ni PFX: se firma en la nube.

**Plan de prueba ya ejecutado:** cert self-signed "Reyes Tech Solutions (Prueba)"
(huella `8DE43451…`, PFX en `_build-batches/reyes-test-codesign.pfx`, clave
`R3yesT3ch2026!`) firmó MSI y portable con éxito — la cadena "not trusted" es
lo esperado con self-signed. Copias firmadas de prueba en
`C:\rustdesk\_build-batches\test-signed\`.

**Para acelerar la reputación con OV:** distribuye desde tu web con HTTPS,
acumula descargas/instalaciones, y envía el exe a Microsoft para análisis
(si es falso positivo) desde https://www.microsoft.com/en-us/wdsi/filesubmission.

---

## Resumen de herramientas y versiones

| Herramienta | Versión | Fuente |
|---|---|---|
| Visual Studio 2022 | Desktop C++ | CI |
| Rust | 1.75.0 | `RUST_VERSION` en CI |
| Flutter | 3.24.5 | `FLUTTER_VERSION` en CI |
| vcpkg | commit `9e593bb…` (2026.07.29) | `VCPKG_COMMIT_ID` en CI |
| Python | 3.12+ | scripts |
| flutter_rust_bridge_codegen | 1.80.1 | `bridge.yml` |
| cargo-expand | 1.0.95 | `bridge.yml` |

---

## Problemas comunes

| Síntoma | Solución |
|---|---|
| `link.exe not found` | Abre el Developer PowerShell de VS 2022, no PowerShell normal |
| vcpkg falla en ffmpeg/nasm | `winget install Gyan.FFmpeg` + asegúrate de tener CMake (viene con VS) |
| `flutter pub get` falla con `extended_text` | Baja a `13.0.0` en pubspec.yaml |
| El bridge no genera | Usa el Developer PowerShell (necesita `cl.exe` para expandir macros) |
| `error: Microsoft Visual C++ 14.0 or greater is required` | Paso 1 no completo (carga de trabajo C++) |
| El exe arranca sin tu marca | Revisa que copiaste `src/common.rs` (la función `apply_reyes_tech_branding`) |
| ffigen: `Couldn't find dynamic library... libclang` | Instala LLVM (solo hace falta `libclang.dll`) en `C:\llvm-15.0.6\bin`, define `LIBCLANG_PATH` y añade `--llvm-path "C:\llvm-15.0.6"` al comando de `flutter_rust_bridge_codegen` |
| hwcodec: `fatal error C1083: libavcodec/avcodec.h` | Faltan libs estáticas: `cd C:\vcpkg && .\vcpkg.exe install ffnvcodec amd-amf --triplet x64-windows-static` y luego `ffmpeg` con `--overlay-ports C:\rustdesk\res\vcpkg\ffmpeg --triplet x64-windows-static` |
| `WIX0150: Undefined preprocessor variable` al reconstruir el MSI | `preprocess.py` reescribe `res/msi` in situ: restaura antes de cada pasada con `git checkout -- res/msi` (conservando tus cambios) |
| `nuget restore`: `Unable to find version '4.0.5' of package 'WixToolset...'` | Falta la fuente: `nuget sources Add -Name nuget.org -Source https://api.nuget.org/v3/index.json` |
| `version ... no se reconoce como comando` en preprocess.py | Corregido en tu copia: se cita la ruta del exe (`f'"{dist_app}" {args}'`) porque el nombre lleva espacios |
| El portable dice "RustDesk" en sus propiedades | Los metadatos salen de `libs/portable/build.rs` y `Cargo.toml` (description); en tu copia ya dicen "Reyes Tech Solutions" — conserva esos archivos al actualizar |
