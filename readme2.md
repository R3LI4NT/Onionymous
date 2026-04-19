<div align="center">

# HackTools GUI

Interfaz gráfica unificada para la suite interna HackTools del equipo de Red Team de Datasec.

Agrupa en una sola aplicación los conversores a Faraday, el agrupamiento de vulnerabilidades, la generación de informes y las utilidades sobre la API de Faraday.

![status](https://img.shields.io/badge/status-internal-00ff7f?style=flat-square)
![python](https://img.shields.io/badge/python-3.11%2B-00ff7f?style=flat-square)
![platform](https://img.shields.io/badge/platform-windows-00ff7f?style=flat-square)
![license](https://img.shields.io/badge/license-internal-informational?style=flat-square)

</div>

---

## Contenido

- [Características](#características)
- [Capturas](#capturas)
- [Requisitos](#requisitos)
- [Instalación](#instalación)
- [Construcción del ejecutable](#construcción-del-ejecutable)
- [Uso](#uso)
- [Estructura del repositorio](#estructura-del-repositorio)
- [Arquitectura de ejecución](#arquitectura-de-ejecución)
- [Agregar una nueva herramienta](#agregar-una-nueva-herramienta)
- [Personalización](#personalización)
- [Resolución de problemas](#resolución-de-problemas)
- [Contribuir](#contribuir)

---

## Características

- **Navegación lateral** con cuatro módulos: Dashboard, Faraday Tools, Agrupamiento y Generar Informes.
- **Terminal embebida** con tema oscuro, verde neón, cursor parpadeante, soporte de códigos ANSI y entrada interactiva para scripts que piden `input()`.
- **Iconografía vectorial** dibujada con primitivas de `Canvas`, independiente de las fuentes del sistema.
- **Ejecución no bloqueante** de los scripts originales como subprocesos, con streaming línea a línea de `stdout` y `stderr`.
- **Empaquetado con PyInstaller**: un único `.exe` autocontenido que incluye el intérprete de Python y todas las librerías, sin necesidad de tener Python instalado en la máquina destino.
- **Preserva los scripts originales sin modificaciones**: la GUI es una capa encima del proyecto existente.

---

## Capturas

> *Sugerencia: colocar las imágenes en `docs/screenshots/` y referenciarlas aquí.*

| Dashboard | Módulo de categoría | Terminal en ejecución |
| :-------: | :-----------------: | :-------------------: |
| `docs/screenshots/dashboard.png` | `docs/screenshots/faraday.png` | `docs/screenshots/terminal.png` |

---

## Requisitos

| Componente | Versión sugerida | Notas |
|---|---|---|
| Python | 3.11 (también funciona 3.10 / 3.12) | Con Tkinter — incluido en el instalador oficial de Windows. |
| Sistema operativo | Windows 10 / 11 | Probado en `x64`. El build genera un `.exe` nativo. |
| Git | Cualquiera reciente | Necesario para `GitPython` y el chequeo de commits contra GitLab. |
| VPN corporativa | — | Obligatoria para los scripts que consultan GitLab (agrupamiento, informes, traductores). |
| Visual C++ Redistributable | x64 reciente | Requerido en la máquina destino para el `.exe`. |

El `.exe` final pesa entre 80 MB y 120 MB según versiones de dependencias. No embebe las carpetas `agrupamiento/`, `faraday_tools/`, `report_builder/`, `diccionario/` ni `traductor/`, por lo que **debe convivir en la misma carpeta** que ellas.

---

## Instalación

Clonar el repositorio con submódulos y cambiar al directorio:

```bash
git clone <url-del-repo> HackTools
cd HackTools
```

Crear un entorno virtual e instalar dependencias:

```bash
python -m venv .venv
.venv\Scripts\activate
pip install -r requirements-gui.txt
```

Ejecutar la GUI en modo desarrollo:

```bash
python HackToolsGUI.py
```

---

## Construcción del ejecutable

### Opción A — script automatizado (recomendada)

Desde la raíz del proyecto, doble click en `build_exe.bat` o desde `cmd`:

```bat
build_exe.bat
```

El script realiza los siguientes pasos:

1. Crea un entorno virtual aislado en `.venv-build/`.
2. Instala las dependencias de `requirements-gui.txt`.
3. Ejecuta PyInstaller con la configuración de `HackToolsGUI.spec`.
4. Copia el binario resultante a la raíz como `HackToolsGUI.exe`.

### Opción B — manual

```bat
python -m venv .venv-build
.venv-build\Scripts\activate
pip install -r requirements-gui.txt
pyinstaller --clean --noconfirm HackToolsGUI.spec
copy dist\HackToolsGUI.exe HackToolsGUI.exe
```

### Resultado

Al finalizar el build, la carpeta HackTools queda con esta distribución:

```
HackTools/
├── HackToolsGUI.exe         ← binario generado
├── agrupamiento/
├── faraday_tools/
├── report_builder/
├── diccionario/
├── traductor/
└── ...
```

El `.exe` debe **permanecer en la raíz de HackTools**. Al moverlo fuera pierde la referencia a los módulos y no puede ejecutar ninguna herramienta.

---

## Uso

1. Conectarse a la VPN corporativa.
2. Abrir `HackToolsGUI.exe` (o lanzar `python HackToolsGUI.py` en modo desarrollo).
3. Seleccionar un módulo del menú lateral.
4. Click en la tarjeta de la herramienta deseada y luego en **EXECUTE**.
5. Seguir las indicaciones de la terminal embebida. Cuando un script requiere entrada (por ejemplo `Presione Enter...` o una opción numérica), escribir la respuesta en el campo inferior del terminal y presionar `Enter`.

### Módulos disponibles

| Módulo | Descripción |
|---|---|
| **Faraday Tools** | Conversores de escaneos (Acunetix, Nessus, ScubaGear, ADAudit, Host Discovery) y utilidades sobre la API de Faraday (subir CSV, limpiar assets, migrar instancias, etc.). |
| **Agrupamiento** | Agrupación de vulnerabilidades por diccionario, mantenimiento del TAD Diccionario y TAD Traductor, traductor de GoPhish, combinación de CSVs. |
| **Generar Informes** | Generación de informes Word a partir de Faraday, planilla técnica con EPSS, gráficas de severidad, Beygoo Stealer y sandbox de templates. |

---

## Estructura del repositorio

```
HackTools/
├── HackToolsGUI.py            # Entry point (modo GUI o puente --run-script)
├── HackToolsGUI.spec          # Configuración de PyInstaller
├── build_exe.bat              # Script de build para Windows
├── requirements-gui.txt       # Dependencias consolidadas
├── README.md                  # Este archivo
│
├── gui/                       # Código fuente de la interfaz
│   ├── app.py                 # Ventana principal
│   ├── theme.py               # Paleta, tipografías, banner ASCII
│   ├── assets/
│   │   └── logo.ico
│   ├── core/
│   │   ├── registry.py        # Catálogo de herramientas y categorías
│   │   └── runner.py          # Gestor de subprocesos
│   ├── pages/
│   │   ├── dashboard.py
│   │   └── category.py
│   └── widgets/
│       ├── sidebar.py
│       ├── terminal.py
│       ├── tool_card.py
│       ├── gradient_header.py
│       ├── matrix_rain.py
│       ├── scrollable.py
│       └── icons.py           # Iconografía vectorial
│
├── agrupamiento/              # Submódulo existente (sin cambios)
├── faraday_tools/             # Submódulo existente (sin cambios)
├── report_builder/            # Submódulo existente (sin cambios)
├── diccionario/
└── traductor/
```

---

## Arquitectura de ejecución

La GUI no importa los scripts de HackTools: los **lanza como subprocesos**, preservando el comportamiento original sin acoplarse al código existente.

```
 ┌────────────────────┐            ┌──────────────────────┐
 │  HackToolsGUI.exe  │ subprocess │  script hijo .py     │
 │  (modo GUI)        ├───────────▶│  (agrupar_vulns.py,  │
 │                    │   pipes    │   generar_informe.py │
 │  tkinter + sidebar │◀───────────┤   etc.)              │
 │  terminal          │  stdout    │                      │
 └────────────────────┘            └──────────────────────┘
```

### Modo puente (`--run-script`)

Cuando el binario está empaquetado con PyInstaller, el subproceso es el **propio ejecutable** invocado con la bandera `--run-script <ruta>`. El entry point detecta la bandera y ejecuta el `.py` pedido dentro del intérprete bundleado, reutilizando todas las librerías incluidas en el `.exe`. Esto permite distribuir la aplicación a máquinas sin Python.

### Resolución del directorio de trabajo

Cada submódulo (`agrupamiento/`, `faraday_tools/`, `report_builder/`) es un repositorio Git independiente. Los scripts originales hacen `open(".git/HEAD")` para verificar la versión local contra GitLab, por lo que el runner fija el CWD del subproceso en la primera carpeta ancestro del script que contenga un `.git/`.

### Encoding

El runner fuerza `UTF-8` en `stdin`, `stdout` y `stderr` del subproceso usando `reconfigure()`, previniendo problemas con caracteres acentuados en Windows (donde la codepage por defecto es `cp1252`).

---

## Agregar una nueva herramienta

1. Colocar el script `.py` dentro de la carpeta correspondiente (por ejemplo `faraday_tools/mi_script.py`).
2. Registrarlo en `gui/core/registry.py` dentro de la categoría adecuada:

```python
   Tool(
       id="mi_script",
       title="Mi Nueva Herramienta",
       description="Qué hace, en una línea.",
       script="faraday_tools/mi_script.py",
       icon="bolt",          # Ver gui/widgets/icons.py para iconos disponibles
       needs_git=True,       # False si el script no depende de GitLab
   ),
```

3. (Opcional) Si la herramienta requiere un icono que todavía no existe, agregarlo en `gui/widgets/icons.py` dentro del diccionario `ICONS`.
4. Reiniciar la GUI. La tarjeta aparece automáticamente.

### Iconos disponibles

| Icono | Nombre |
|---|---|
| 🏠 | `home` |
| ⚡ | `bolt` |
| ⚛ | `atom` |
| 📄 | `document` |
| 🔄 | `convert`, `exchange`, `transfer` |
| 🛡 | `shield` |
| 💧 | `drop` |
| 👤 | `user` |
| 📡 | `radar`, `satellite` |
| 🧬 | `cluster`, `merge`, `layers` |
| ⬆ | `upload` |
| 🗑 | `trash`, `broom` |
| 🔤 | `translate` |
| ⚙ | `gear` |
| ☠ | `skull` |
| 📚 | `book`, `stack` |
| 📧 | `mail` |
| 📊 | `file_chart`, `spreadsheet`, `chart_bar`, `chart_pie` |
| ⚗ | `flask` |

---

## Personalización

### Paleta de colores

Todas las constantes visuales están en `gui/theme.py`:

```python
COLORS = {
    "green_neon":   "#00ff7f",   # Verde principal
    "green_bright": "#39ff6a",   # Verde brillante (hover, énfasis)
    "bg_root":      "#05070a",   # Fondo general
    "bg_card":      "#0c130e",   # Fondo de tarjetas
    ...
}
```

### Tipografía

La GUI usa **Consolas** como fuente monoespaciada por defecto. Para cambiarla, editar `FONT_FAMILY_MONO` en `gui/theme.py`.

### Ícono del ejecutable

El `.exe` usa `gui/assets/logo.ico`. Para cambiarlo, reemplazar el archivo y volver a construir con `build_exe.bat`.

---

## Resolución de problemas

| Síntoma | Causa | Solución |
|---|---|---|
| El `.exe` abre pero no ejecuta ninguna herramienta. | Fue movido fuera de la carpeta HackTools. | Devolverlo a la raíz, junto a `agrupamiento/`, `faraday_tools/`, etc. |
| *"No se puede conectar con el GitLab de Datasec"* | Sin VPN o token caducado. | Conectarse a la VPN. Si el token venció, regenerarlo en GitLab y actualizar `REPO_TOKEN` en `report_builder/constantes/conexiones.py`. |
| *"ATENCIÓN: Su repositorio se encuentra desactualizado."* | El repo local tiene commits viejos. | Ejecutar `git restore . && git pull` en el submódulo afectado. |
| Acentos mal renderizados (`?`, `�`) en la terminal. | Encoding del subproceso distinto de UTF-8. | Verificar que `HackToolsGUI.py` esté actualizado con el reconfigure de `sys.stdout`. |
| El `.exe` cierra sin ventana y sin error. | Falta VC++ Redistributable en la máquina destino. | Instalar **Microsoft Visual C++ Redistributable (x64)**. |
| PyInstaller falla con `ModuleNotFoundError` durante el build. | La dependencia no está en el `requirements-gui.txt` o no se auto-detecta. | Agregarla tanto al requirements como a `hiddenimports` dentro de `HackToolsGUI.spec`. |
| Los iconos del sidebar no se ven. | Bug conocido si el `icon_holder` no tiene `fill=tk.Y`. | Verificar que esté empaquetado con `.pack(side=tk.LEFT, fill=tk.Y)`. |

---

## Contribuir

### Flujo de trabajo

1. Crear una rama a partir de `master` (o `develop` si aplica): `git checkout -b feature/mi-cambio`.
2. Realizar los cambios y verificarlos en modo desarrollo: `python HackToolsGUI.py`.
3. Validar que el build compile: `build_exe.bat`.
4. Crear un Merge Request en GitLab, describiendo el cambio y adjuntando capturas si es una mejora visual.

### Convenciones

- Código en **Python 3.11** con type hints donde aporte claridad.
- Comentarios y strings en español; identificadores en inglés.
- Mantener los scripts originales intactos. Cualquier modificación funcional debe hacerse en `gui/` o en la carpeta propietaria del script.
- No agregar nuevas dependencias sin justificación; si es necesario, actualizar `requirements-gui.txt` y `HackToolsGUI.spec` en el mismo commit.

---

<div align="center">

**HackTools GUI** · Datasec Red Team · Uso interno

</div>
