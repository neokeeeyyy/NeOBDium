![AppImage](/docs/banner2.jpg)

<div align="center">
  <h1>NeOBDium</h1>
  <b>Diagnóstico vehicular profesional en español.</b>
  <p><em>Fork en español de OBDium. Herramienta de diagnóstico vehicular basada en Rust que se conecta con adaptadores ELM327, ofreciendo datos OBD-II en vivo, análisis de códigos de falla y decodificación VIN offline.</p></em>

  <img src="https://img.shields.io/badge/version-1.5.1-blue" alt="Version">
  <img src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-blue" alt="Platform">
  <img src="https://img.shields.io/badge/license-GPL--3.0-purple" alt="License">

   <a href="#acerca-de">Acerca de</a>
   ·
   <a href="#descargar">Descargar</a>
   ·
   <a href="#inicio-rápido">Inicio rápido</a>
   ·
   <a href="#compilar-desde-código">Compilar desde código</a>
   ·
   <a href="#contribuir">Contribuir</a>

</div>

<details>
   <summary><b>¡Haz clic para ver capturas de pantalla!</b></summary>

   ![connect-screen](/examples/connect-screen.png)
   <p align="center"><em>Pantalla de Conexión - No Conectado</em></p>

   ![connected-screen](/examples/connected-screen.png)
   <p align="center"><em>Pantalla de Conexión - Conectado</em></p>

   ![obd-overview](/examples/obd-overview-screen.png)
   <p align="center"><em>Panel OBD</em></p>

   ![readiness-tests](/examples/readiness-tests-screen.png)
   <p align="center"><em>Estado de Pruebas I/M</em></p>

   ![supported-pids](/examples/supported-pids-screen.png)
   <p align="center"><em>Lista de PIDs Soportados</em></p>

   ![vin-decoding](/examples/vin-decoding-screen.png)
   <p align="center"><em>NeOBDium Decodificador VIN</em></p>

   ![settings](/examples/settings-screen.png)
   <p align="center"><em>Pantalla de Preferencias</em></p>

</details>

## Tabla de Contenidos

- [Acerca de](#acerca-de)
- [Características](#características)
- [Diferencias con OBDium](#diferencias-con-obdium)
- [Descargar](#descargar)
- [Inicio rápido](#inicio-rápido)
- [Compilar desde código](#compilar-desde-código)
- [Implementación y Lógica](#implementación-y-lógica)
- [Contribuir](#contribuir)
- [Licencia](#licencia)
- [Créditos](#créditos)

## Acerca de

**NeOBDium** es un fork en español de [OBDium](https://github.com/provrb/obdium), una herramienta de diagnóstico vehicular rápida, moderna y extensible basada en Rust. Se conecta con adaptadores ELM327 vía serie para acceder a datos de sensores en vivo, diagnósticos profundos y decodificación precisa de VIN, todo completamente offline y con la interfaz traducida al español.

El objetivo de NeOBDium es llevar OBDium a la comunidad hispanohablante con una interfaz completamente traducida y un rediseño visual profesional orientado a usos técnicos.

## Características

- **Códigos de Error:** Lee códigos de diagnóstico de fallas (DTC) de **Powertrain, Carrocería, Chasis** y **Red** con descripción incluida.
- **Métricas en Vivo:** Lee y decodifica varios PIDs OBD-II (motor, combustible, aire, escape, diagnósticos, etc.).
- **Decodificación VIN Avanzada:** Decodificación detallada del VIN usando un parser personalizado y búsquedas SQLite basadas en la base de datos VPIC de la NHTSA.
- **Pruebas I/M:** Verifica que los sistemas de emisiones de tu vehículo funcionen correctamente.
- **Interfaz Moderna:** Desarrollada con Tauri + JS/HTML/CSS. Interfaz profesional en modo oscuro con tipografía JetBrains Mono.
- **Multiplataforma:** Disponible para macOS, Windows 10/11 y Linux.
- **Modo Demo:** Simula datos OBD-II en vivo sin necesidad de un vehículo real.

## Diferencias con OBDium

- **Completamente en español:** Interfaz de usuario, notificaciones, etiquetas y documentación traducidas al español.
- **Rediseño UI/UX profesional:** Nueva paleta de colores, tipografía JetBrains Mono, layout compacto y aspecto de herramienta de diagnóstico profesional.
- **Compatibilidad con Void Linux:** Workflow de CI para compilar en Void Linux vía GitHub Actions.
- **Adaptado para la comunidad hispanohablante.**

## Descargar

Dirígete a la [sección de releases](https://github.com/neokeeeyyy/NeOBDium/releases) para descargar la última versión.

## Inicio rápido

Para una guía de usuario completa, consulta el [manual de usuario](https://provrb.github.io/obdium/) (en inglés, aplica igual).

### Conectarse a un vehículo

1. Ejecuta la aplicación.
2. Conecta tu adaptador ELM327 al puerto OBD-II del vehículo y a tu computadora.
3. Enciende el encendido o arranca el vehículo.
4. Ve al panel **Conexión**:
   - Selecciona tu puerto serie. Si no aparecen puertos, haz clic en el botón de refrescar.
   - Opcionalmente cambia el protocolo OBD-II y la velocidad de transmisión.
5. Haz clic en **Conectar** y espera la notificación de conexión exitosa.

### Usar cada función

- Los datos OBD se registran en el **Panel OBD**.
- Revisa códigos de falla en el panel **Códigos de Error**.
- Visualiza datos en vivo en los **Gráficos**.
- Decodifica un VIN en el panel **Decodificación de VIN**.
- Para dejar de rastrear una métrica, haz clic en su tarjeta en el **Panel OBD**; haz clic de nuevo para reanudar.
- Modifica preferencias en el panel **Preferencias**.
- Explora el índice de PIDs en **Ver PIDs**.
- Agrega PIDs personalizados desde **Ver PIDs**.

### Modo Demo

El modo demo reproduce respuestas grabadas para simular datos OBD-II en vivo sin vehículo.

Para activarlo:
1. Descarga un archivo de respuestas grabadas (como el [requests.json original](https://github.com/provrb/obdium/blob/main/backend/data/requests.json)).
2. Colócalo en la carpeta `data`.
3. Ve a **Conexión**, selecciona **MODO DEMO** en el puerto serie.
4. Haz clic en **Conectar**.

## Compilar desde código

<details>
   <summary><b>Instrucciones de compilación</b></summary>

### Requisitos previos

1. **Instalar Rust** desde [rust-lang.org](https://www.rust-lang.org/tools/install).

2. **Instalar Tauri CLI:**
   ```sh
   cargo install tauri-cli --version "^2.0.0"
   ```

3. **Clonar el repositorio:**
   ```sh
   git clone https://github.com/neokeeeyyy/NeOBDium.git
   cd NeOBDium
   ```

### Compilar

```sh
cargo tauri build
```

Los binarios se ubicarán en:
- Linux: `backend/target/release/bundle/deb`, `rpm` o `appimage`
- Windows: `backend/target/release/bundle/msi` o `nsis`
- macOS: `backend/target/release/bundle/dmg`

### Modo desarrollo

```sh
cargo tauri dev --no-watch
```

### Compilar en Void Linux

```sh
sudo xbps-install -S rust cargo libwebkit2gtk41-devel gtk+3-devel glib-devel cairo-devel pango-devel gdk-pixbuf-devel librsvg-devel libsoup3-devel atk-devel
cd backend
cargo build --release
```

También puedes usar el workflow de GitHub Actions incluido (`.github/workflows/build-void.yml`) para compilar automáticamente en Void Linux.

</details>

## Implementación y Lógica

Este proyecto requirió investigación extensa sobre ELM327, el protocolo OBD-II y decodificación de respuestas. El crédito de la implementación original pertenece a [provrb](https://github.com/provrb).

<details>
   <summary><b>Lógica de implementación</b></summary>

   1. **Comunicación en tiempo real** a través del struct [OBD](backend/src/obd.rs). NeOBDium establece una conexión serie con el adaptador ELM327 y solicita datos usando PIDs. Por ejemplo, para obtener la temperatura del refrigerante del motor: `0105`.

   2. **Respuestas** del vehículo en cadenas hexadecimales. Los bytes 'A' a 'E' se usan con ecuaciones específicas para calcular el resultado. Ver [response.rs](backend/src/response.rs). Para temperatura del refrigerante: `A - 40`.

   3. **Decodificación VIN**: Implementación propia basada en ingeniería inversa de la base de datos [vPIC de la NHTSA](https://vpic.nhtsa.dot.gov/api/), adaptada a SQLite y Rust.

   Para preguntas sobre la implementación, abre un Issue o Discussion en el repositorio.
</details>

## Contribuir

Las contribuciones son bienvenidas. Por favor revisa el archivo [CONTRIBUTING](CONTRIBUTING.md) para más información.

## Licencia

Este proyecto está licenciado bajo GPL-3.0. Ver [`LICENSE`](LICENSE) para más detalles.

## Créditos

NeOBDium es un fork de [OBDium](https://github.com/provrb/obdium) por [provrb](https://github.com/provrb). Todo el crédito del motor de diagnóstico original y la lógica OBD-II pertenece al proyecto original. Este fork se enfoca en la traducción al español y mejoras en la interfaz de usuario.
