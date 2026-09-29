# Avisos de terceros (Third-Party Notices)

Reyes Tech Solutions Remote Desktop 1.5.0 se basa en software de terceros.
Este archivo resume los componentes incluidos en esta distribución binaria,
con sus titulares de derechos y licencias.

---

## RustDesk (base de esta aplicación)

Copyright (C) 2022-2026 Purslane Ltd. y contribuidores del proyecto RustDesk
(https://github.com/rustdesk/rustdesk)

Licencia: **GNU Affero General Public License v3.0 (AGPLv3)**.
El texto completo de la licencia se incluye en el archivo `LICENSE.txt` que
acompaña a esta aplicación y en https://www.gnu.org/licenses/agpl-3.0.html

Este programa es software libre: puede redistribuirlo y/o modificarlo bajo los
términos de la AGPLv3. Este programa se distribuye con la esperanza de que sea
útil, pero SIN NINGUNA GARANTÍA; sin siquiera la garantía implícita de
COMERCIABILIDAD o APTITUD PARA UN PROPÓSITO PARTICULAR. Véase `LICENSE.txt`.

Conforme a la AGPLv3, el código fuente completo y correspondiente de la
versión 1.5.0 de esta distribución está disponible en:
https://github.com/ReyesTechSolutions/rustdesk-1.5.0-rtx

## Componentes compilados dentro de la aplicación (vía vcpkg)

| Componente | Titular principal | Licencia |
|---|---|---|
| FFmpeg (libavcodec, libavformat, libavutil) | FFmpeg contributors | LGPL-2.1+ / **GPL-2.0+** (compilado con `--enable-gpl` y decodificadores H.264/HEVC) |
| libvpx | Google Inc. y otros | BSD-3-Clause |
| libyuv | The libyuv authors (Google) | BSD-3-Clause |
| libaom | Google Inc. y otros | BSD-2-Clause + Alliance for Open Media Patent License 1.0 |
| opus | Xiph.Org Foundation / Skype / Microsoft | BSD-3-Clause |
| libjpeg-turbo | The libjpeg-turbo Project Authors | BSD-3-Clause (IJG compatible) |
| mfx-dispatch / Intel Media SDK | Intel Corporation | MIT |
| AMD AMF | Advanced Micro Devices | MIT |
| ffnvcodec headers (NVENC/NVDEC) | NVIDIA Corporation | MIT |
| libsodium | Frank Denis y contribuidores | ISC |
| zlib | Jean-loup Gailly, Mark Adler | zlib |

Las licencias completas de estos componentes se distribuyen con sus fuentes
(véase el repositorio indicado arriba; los ports de vcpkg aplicados están en
`res/vcpkg/` del mismo repositorio). Conforme a la GPL-2.0+ aplicable a FFmpeg
en esta configuración, el código fuente de FFmpeg utilizado está disponible en
el repositorio de código fuente indicado arriba.

## Motor y librerías de interfaz

| Componente | Titular principal | Licencia |
|---|---|---|
| Flutter engine (flutter_windows.dll) | Google LLC | BSD-3-Clause (+ FreeType, HarfBuzz, etc. en sus propios términos) |
| Dart runtime | Google LLC | BSD-3-Clause |
| Plugins Flutter de RustDesk y de terceros (desktop_drop, screen_retriever, texture_rgba_renderer, etc.) | Sus autores | MIT / BSD / Apache-2.0 |
| flutter_rust_bridge | fzyzcjy y contribuidores | MIT / Apache-2.0 |
| Rust, librerías estándar y crates de crates.io | The Rust Project Developers y autores respectivos | MIT / Apache-2.0 (u otras permisivas por crate) |

## Windows runner y recursos

| Componente | Titular | Licencia |
|---|---|---|
| runner de Flutter Windows (C++, plantilla) | Google LLC | BSD-3-Clause |
| usbmmidd_v2 (driver de pantallas virtuales, opcional en despliegues CI) | Amyuni / RustDesk packaging | según su distribución oficial |

---

### Cómo obtener el código fuente completo

AGPLv3 §13 y las licencias BSD/MIT de los componentes exigen poner el código
fuente a disposición de quien recibe los binarios:

**https://github.com/ReyesTechSolutions/rustdesk-1.5.0-rtx**

Incluye: árbol fuente 1.5.0 + modificaciones Reyes Tech Solutions +
ports vcpkg + instrucciones de compilación (GUIA-COMPILACION-REYES-TECH.md).

### Marca

"RustDesk" y su logotipo son marcas de Purslane Ltd. "Reyes Tech Solutions" y
su logotipo son marcas de Reyes Tech Solutions. La marca no está cubierta por
la AGPLv3; los archivos de marca de RustDesk fueron sustituidos íntegramente
en esta distribución.

Reyes Tech Solutions — https://reyestechsolutions.netlify.app/
soporte: reyes.tech.solutionss@gmail.com · +52 999 332 9868
