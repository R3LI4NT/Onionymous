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
