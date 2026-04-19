<div align="center">

# HackTools GUI

Interfaz gráfica unificada para la suite interna HackTools del equipo de Red Team de Datasec.

Agrupa en una sola aplicación los conversores a Faraday, el agrupamiento de vulnerabilidades, la generación de informes y las utilidades sobre la API de Faraday.

![status](https://img.shields.io/badge/status-internal-2A6625?style=flat-square)
![python](https://img.shields.io/badge/python-3.11%2B-2A6625?style=flat-square)
![platform](https://img.shields.io/badge/platform-windows-2A6625f?style=flat-square)

</div>

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
