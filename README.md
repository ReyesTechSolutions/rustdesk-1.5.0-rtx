# Reyes Tech Solutions — distribución 1.5.0 (basado en RustDesk)

Repositorio de código fuente de **Reyes Tech Solutions Remote Desktop 1.5.0**,
una distribución personalizada del cliente de escritorio remoto RustDesk.

- **Software base:** RustDesk 1.5.0 — https://github.com/rustdesk/rustdesk
- **Titular del proyecto base:** Copyright (C) 2022–2026 Purslane Ltd. y contribuidores
- **Licencia:** GNU Affero General Public License v3.0 — véase [`LICENCE`](LICENCE)
  (copia original del upstream) y [`LICENSE.txt`](LICENSE.txt) (copia incluida con los
  binarios). Este fork conserva íntegramente la licencia del proyecto original.
- **Avisos de terceros:** [`THIRD-PARTY-NOTICES.md`](THIRD-PARTY-NOTICES.md)
- **Autor de la distribución:** Reyes Tech Solutions — https://reyestechsolutions.netlify.app/

---

## Qué cambió respecto a RustDesk 1.5.0

Personalización de marca y menores (sin cambios de protocolo ni de servidores):

| Área | Cambio |
|---|---|
| Identidad | Nombre de aplicación "Reyes Tech Solutions" en tiempo de ejecución (`src/common.rs`, `apply_reyes_tech_branding()`) |
| Esquema URI | `reyestechsolutions://` además del `rustdesk://` original (iOS/macOS/Android/Linux) |
| Interfaz | Paleta propia (indigo/violeta/cian, temas claro y oscuro) en `flutter/lib/common.dart` y pestañas |
| Textos | i18n es/en adaptados a la marca (claves intactas) |
| Iconos y marca gráfica | Todos los iconos, logos, splash y assets de marca reemplazados por los de Reyes Tech (Windows, macOS, Linux, Android, iOS) |
| Windows | Metadatos del ejecutable (Runner.rc, winres) con atribución a Purslane Ltd. según AGPLv3 |
| MSI | Fabricante/nombre de producto "Reyes Tech Solutions"; `preprocess.py` con rutas citadas (nombres con espacios) |
| Legal | `LICENSE.txt` y `THIRD-PARTY-NOTICES.md` incluidos con la distribución |
| About | Enlaces web/privacidad/código fuente de Reyes Tech y atribución del proyecto base |

**Sin cambios:** lógica de conexión, protocolo, cifrado, servidores por defecto
(se usan los públicos de RustDesk) y comportamiento general del programa.

## Estructura relevante

```
LICENCE                       Licencia AGPLv3 (verbatim del upstream)
LICENSE.txt                   Copia de la licencia distribuida con los binarios
THIRD-PARTY-NOTICES.md        Atribuciones de terceros (ffmpeg GPL, BSD, ISC, Apache...)
REYES-TECH-FORK.md            Este archivo
GUIA-COMPILACION-REYES-TECH.md  Instrucciones de compilación completas (Windows x64)
libs/hbb_common/              Submódulo upstream (rustdesk/hbb_common)
res/vcpkg/                    Ports vcpkg usados por la CI upstream (aom, ffmpeg, ...)
```

## Cómo compilar (Windows x64)

Ver [`GUIA-COMPILACION-REYES-TECH.md`](GUIA-COMPILACION-REYES-TECH.md). Resumen:

1. Visual Studio 2022 (carga de trabajo C++) + Rust 1.75.0 + Flutter 3.24.5
   (con el engine custom indicado en la guía) + Python 3.12 + vcpkg en el commit
   `9e593bb18ea69cc5095e012465dcd675a822ed0d`.
2. `flutter pub get` en `flutter/` (con `extended_text: 13.0.0`).
3. Generar el puente con `flutter_rust_bridge_codegen 1.80.1` (comando exacto en la guía).
4. `vcpkg install --triplet x64-windows-static` con el `vcpkg.json` del repo.
5. `python build.py --flutter --hwcodec --vram --portable --skip-portable-pack`.
6. MSI: `res/msi/preprocess.py` + `nuget restore` + `msbuild` (comandos en la guía).

## Cumplimiento AGPLv3

Esta distribución se ofrece bajo los términos de la AGPLv3, sin garantía alguna.
El fuente completo correspondiente a cada binario publicado por Reyes Tech Solutions
está disponible en este repositorio (etiqueta/commit correspondiente a la versión).
Para reportes sobre esta distribución: reyes.tech.solutionss@gmail.com
```

---

## README del proyecto base (RustDesk)

El README original de RustDesk —con instrucciones de compilación multiplataforma, estructura del código y capturas— está en [`README-RUSTDESK.md`](README-RUSTDESK.md).
