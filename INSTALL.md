# Guía de Instalación de UMP

UMP (Umbral Package Manager) es el gestor de paquetes oficial del lenguaje Umbral.
Esta guía te ayudará a instalar UMP v1.0.4 en tu sistema operativo.

---

## 📋 Requisitos previos

### Todos los sistemas

- **Rust**: Versión 1.70 o superior
  - Descargar desde: https://rustup.rs/
- **Git**: Para clonar el repositorio
  - Descargar desde: https://git-scm.com/
- **Umbral** (recomendado): UMP gestiona paquetes del lenguaje Umbral
  - Descargar desde: https://github.com/hersac/umbral/releases

### Verificar requisitos

```bash
# Verificar Rust
rustc --version
cargo --version

# Verificar Git
git --version
```

---

## 🐧 Instalación en Linux

### Opción A: Paquete .deb — recomendada

Si se dispone de un paquete compilado (`ump_<versión>_amd64.deb` desde los releases):

```bash
sudo dpkg -i ump_1.0.4_amd64.deb
```

El paquete instala:
- `ump` en `/usr/bin/ump`
- `install.sh` en `/usr/share/ump/install.sh`
- `README.md` en `/usr/share/doc/ump/README.md`

Verifica con:

```bash
ump --version
```

### Opción B: Compilar e instalar desde el código fuente

#### Paso 1: Clonar el repositorio

```bash
git clone https://github.com/hersac/ump.git
cd ump
```

#### Paso 2: Ejecutar el instalador

```bash
chmod +x install.sh
./install.sh
```

El script hará lo siguiente:
1. ✅ Verificará que Rust esté instalado
2. 📦 Compilará UMP en modo release
3. 🚀 Instalará el binario `ump` en `~/.cargo/bin`

#### Paso 3: Configurar PATH (si es necesario)

Si el comando `ump` no se encuentra, agrega esto a tu `~/.bashrc`:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Luego recarga tu shell:

```bash
source ~/.bashrc
```

#### Paso 4: Verificar la instalación

```bash
ump --version
ump --help
```

---

## 🍎 Instalación en macOS

### Paso 1: Instalar Homebrew (si no lo tienes)

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### Paso 2: Instalar Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Paso 3: Clonar e instalar UMP

```bash
git clone https://github.com/hersac/ump.git
cd ump
chmod +x install.sh
./install.sh
```

### Paso 4: Configurar PATH (si es necesario)

Para **bash** (`~/.bash_profile`):

```bash
export PATH="$HOME/.cargo/bin:$PATH"
source ~/.bash_profile
```

Para **zsh** (`~/.zshrc`):

```bash
export PATH="$HOME/.cargo/bin:$PATH"
source ~/.zshrc
```

### Paso 5: Verificar la instalación

```bash
ump --version
ump --help
```

---

## 🪟 Instalación en Windows

### Opción A: Instalador gráfico (.exe) — recomendada

Si se dispone de un instalador compilado (`ump-setup-<versión>.exe`), es la forma más sencilla:

1. Ejecuta `ump-setup-<versión>.exe`
2. En el asistente, selecciona el directorio donde quieres instalar UMP
3. El instalador copia `ump.exe` y agrega automáticamente el directorio al `PATH` del usuario
4. Al finalizar puedes verificar la versión instalada

Para **desinstalar**, usa "Agregar o quitar programas" de Windows o ejecuta `uninstall.exe` en la carpeta de instalación. El desinstalador también elimina la entrada del `PATH`.

### Opción B: Compilar e instalar desde el código fuente

1. Instala Rust (https://rustup.rs/) y Git (https://git-scm.com/download/win)
2. Clona el repositorio:
   ```powershell
   git clone https://github.com/hersac/ump.git
   cd ump
   ```
3. Ejecuta el instalador:
   ```powershell
   PowerShell -ExecutionPolicy Bypass -File install.ps1
   ```
   El script hará lo siguiente:
   1. ✅ Verificará que Rust esté instalado
   2. 📦 Compilará UMP en modo release
   3. 🚀 Instalará el binario en `%USERPROFILE%\.cargo\bin`
   4. ⚙️ Configurará automáticamente el PATH del usuario

### Paso 4: Reiniciar terminal

**IMPORTANTE**: Cierra y vuelve a abrir PowerShell/CMD para que los cambios en el PATH surtan efecto.

### Paso 5: Verificar la instalación

```powershell
ump --version
ump --help
```

### Generar el instalador .exe (para distribución)

Para producir `ump-setup-<versión>.exe` desde el código fuente:

```bash
./windows/build.sh
```

Requisitos:
- **Rust** con el target `x86_64-pc-windows-gnu` (en Windows se usa el nativo; el script agrega el target automáticamente)
- **makensis** (NSIS) — en Linux: `sudo apt install nsis`; en Windows descárgalo de https://nsis.sourceforge.net/

El script compila el binario para Windows, lo copia a `target/release/` y genera el instalador en `windows/`. El asistente permite elegir el directorio de instalación y agrega ese directorio al `PATH` del usuario (HKCU), sin requerir permisos de administrador. El desinstalador elimina el archivo y la entrada del `PATH`.

---

## ✅ Verificar que todo funciona

### Prueba rápida

```bash
ump --version
ump --help
```

### Crear tu primer proyecto

```bash
ump create mi-super-app
cd mi-super-app
# ¡Listo para codificar!
```

Prueba inicializar en un proyecto existente:

```bash
ump init
```

---

## 🔧 Solución de problemas

### Error: "cargo: command not found"

**Causa**: Rust no está instalado o no está en el PATH.

**Solución**:
1. Instala Rust desde https://rustup.rs/
2. Reinicia tu terminal
3. Verifica: `cargo --version`

### Error: "ump: command not found" (después de instalar)

#### Linux/macOS

**Causa**: `~/.cargo/bin` no está en tu PATH.

**Solución**:

1. Verifica tu PATH:
```bash
echo $PATH | grep cargo
```

2. Si no aparece, agrega a tu `~/.bashrc` o `~/.zshrc`:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

3. Recarga el shell:
```bash
source ~/.bashrc  # o source ~/.zshrc
```

#### Windows

**Causa**: El PATH no se actualizó correctamente.

**Solución**:

1. Cierra y vuelve a abrir PowerShell/CMD (IMPORTANTE)

2. Verifica la variable PATH:
```powershell
$env:Path
```

3. Si no aparece `%USERPROFILE%\.cargo\bin`, ejecuta el instalador nuevamente

4. Alternativamente, agrega manualmente al PATH:
   - Busca "Variables de entorno" en el menú de inicio
   - Edita la variable "Path" del usuario
   - Agrega: `%USERPROFILE%\.cargo\bin`

### Error de permisos en Linux/macOS

**Causa**: El script de instalación no tiene permisos de ejecución.

**Solución**:
```bash
chmod +x install.sh
./install.sh
```

### Error: "execution policy" en Windows

**Causa**: La política de ejecución de PowerShell está restringida.

**Solución**:
```powershell
PowerShell -ExecutionPolicy Bypass -File install.ps1
```

O cambia la política permanentemente:
```powershell
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### Error de compilación

**Causa**: Falta alguna dependencia de Rust o hay un problema con el código fuente.

**Solución**:

1. Actualiza Rust:
```bash
rustup update
```

2. Limpia y recompila:
```bash
cargo clean
cargo build --release
```

3. Si persiste, reporta el issue en GitHub con el log completo.

---

## 🔄 Actualización

Para actualizar UMP a la última versión:

```bash
cd ump
git pull origin main
./install.sh  # o install.ps1 en Windows
```

---

## 🗑️ Desinstalación

### Linux / macOS

```bash
cd ump
./uninstall.sh
```

O manualmente:

```bash
cargo uninstall ump
```

Si instalaste desde `.deb`:

```bash
sudo dpkg -r ump
```

### Windows

Si instalaste con el instalador `.exe`, usa "Agregar o quitar programas" o ejecuta `uninstall.exe` en la carpeta de instalación.

Si instalaste desde código fuente:

```powershell
cd ump
PowerShell -ExecutionPolicy Bypass -File uninstall.ps1
```

O manualmente:

```powershell
cargo uninstall ump
```

---

## 🛠️ Instalación desde código fuente (sin script)

Si prefieres instalación manual:

```bash
# 1. Clonar
git clone https://github.com/hersac/ump.git
cd ump

# 2. Compilar
cargo build --release

# 3. Instalar
cargo install --path .

# 4. Verificar
ump --version
ump --help
```

Los binarios se instalarán en:
- Linux/macOS: `~/.cargo/bin/`
- Windows: `%USERPROFILE%\.cargo\bin\`

También puedes mover el binario manualmente:

```bash
# Linux/macOS
sudo mv target/release/ump /usr/local/bin/
```

---

## 📦 Instalación en sistemas sin Rust

Si no puedes instalar Rust, puedes usar los binarios precompilados (cuando estén disponibles):

### Releases

Descarga el binario para tu plataforma desde:
https://github.com/hersac/ump/releases

Extrae y mueve a una ubicación en tu PATH:

**Linux (.deb):**
```bash
sudo dpkg -i ump_1.0.4_amd64.deb
```

**Linux (binario suelto):**
```bash
tar -xzf ump-linux-x64.tar.gz
sudo mv ump /usr/local/bin/
```

**Windows:**
```powershell
# Opción 1: Ejecuta ump-setup-1.0.4.exe
# Opción 2: Extrae el ZIP, mueve ump.exe a C:\Program Files\UMP\
#           y agrega C:\Program Files\UMP\ al PATH
```

---

## 🌐 Instalación en entornos especiales

### Docker

```dockerfile
FROM rust:1.70

WORKDIR /app
RUN git clone https://github.com/hersac/ump.git
WORKDIR /app/ump
RUN cargo install --path .

CMD ["ump", "--help"]
```

### WSL (Windows Subsystem for Linux)

Sigue las instrucciones de Linux dentro de tu distribución WSL.

---

## 📝 Siguiente paso

Una vez instalado, consulta:

- [README.md](./README.md) - Documentación principal de UMP
- [Umbral](https://github.com/hersac/umbral) - Lenguaje de programación Umbral

---

## 💬 ¿Necesitas ayuda?

- **Issues**: https://github.com/hersac/ump/issues
- **Discussions**: https://github.com/hersac/ump/discussions

---

**¡Disfruta gestionando paquetes de Umbral! 🎉**
