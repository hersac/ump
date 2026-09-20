# UMP (Umbral Package Manager) 🚀

¡Bienvenido a **UMP**, el compañero esencial para desarrollar en **Umbral**!

Gestiona tus proyectos, instala librerías y automatiza tareas con una herramienta diseñada para ser rápida, simple y potente. Si estás probando el lenguaje Umbral, **UMP** es la pieza que te falta para llevar tus ideas al siguiente nivel.

---

## 🛠️ Instalación

Antes de empezar, asegúrate de tener el entorno listo.

### 1. Instala el Lenguaje Umbral
Para usar UMP, primero necesitas el motor de Umbral. Ve al repositorio oficial **[hersac/umbral](https://github.com/hersac/umbral)** y descarga la última versión:

- **🐧 Linux**: Descarga e instala el paquete `.deb`.
- **🪟 Windows**: Descarga y ejecuta el instalador `.exe` (`umbral-setup-<versión>.exe`).
- **⚙️ Desde código**: Si prefieres compilarlo tú mismo, clona el repo y sigue las instrucciones de compilación.

### 2. Instala UMP (Versión 1.1.0)
Una vez tengas Umbral, consigue el gestor de paquetes oficial:

- **🐧 Linux**: Descarga el paquete `.deb` desde los [releases](https://github.com/hersac/ump/releases) e instálalo con `sudo dpkg -i ump_<versión>_amd64.deb`.
- **🪟 Windows**: Descarga y ejecuta el instalador `ump-setup-<versión>.exe` desde los [releases](https://github.com/hersac/ump/releases).
- **🦀 Código Fuente**:
  ```bash
  git clone https://github.com/hersac/ump.git
  cd ump
  chmod +x install.sh
  ./install.sh
  # En Windows: PowerShell -ExecutionPolicy Bypass -File install.ps1
  # O manual: cargo install --path .
  ```

> 📖 Guía completa con todas las opciones (paquete `.deb`, instalador `.exe`, compilación manual, Docker, solución de problemas): ver [INSTALL.md](./INSTALL.md).

---

## 🚀 ¡Empieza a crear!

### Crea tu primer proyecto
Olvídate de configurar carpetas a mano. Genera un proyecto listo para funcionar en segundos:

```bash
ump create mi-super-app
cd mi-super-app
# ¡Listo para codificar!
```

### Inicializa en un proyecto existente
¿Ya tienes código? Hazlo compatible con UMP al instante:

```bash
ump init
```

### Potencia tu código con librerías explicita
Instala dependencias de forma rápida y segura. Todo queda registrado en tu `umpkg.yml`:

```bash
ump add http math
```

Instala una versión concreta (pide confirmación y valida el Umbral requerido):

```bash
ump add http@1.1.0
```

¿Cambiaste de opinión? Elimínalas igual de fácil:

```bash
ump remove http
```

### Ejecuta y automatiza
Corre tus scripts definidos de manera sencilla:

```bash
ump run start    # Levanta tu aplicación
ump run dev      # Modo desarrollo
ump run test     # Ejecuta tus pruebas
```

### Revisa y aplica actualizaciones
Consulta qué dependencias tienen versiones nuevas (solo informa):

```bash
ump update
```

Actualiza indicando la dependencia. Sin versión sube a la última;
con `@versión` sube a esa versión concreta (siempre con confirmación
y validando la versión de Umbral compatible):

```bash
ump upgrade http          # última disponible
ump upgrade http@1.1.0    # versión concreta
```

---

## 📄 Tu proyecto, bajo control (`umpkg.yml`)

Todo lo que tu proyecto necesita está en un solo lugar, claro y legible:

```yaml
name: mi-super-app
version: 1.0.0
description: "La próxima gran cosa escrita en Umbral"
umbral: ">=0.1.0"
main: src/main.um

scripts:
  start: umbral src/main.um
  dev: umbral src/main.um --watch
  test: umbral tests/main.um

dependencies:
  http: "^1.0.0"
  math: "^1.0.0"
```

---
## 🤝 ¡Únete a nosotros!

Este proyecto es open source y amamos las contribuciones. Si quieres ser parte de la historia de Umbral:

1.  **Haz un Fork**: Crea tu propia copia del repositorio.
2.  **Crea una Rama**: `git checkout -b feature/mi-nueva-feature`
3.  **Haz Cambios Brillantes**: Implementa tus mejoras. ¡Asegúrate de que todo compile!
4.  **Sube tus Cambios**: `git push origin feature/mi-nueva-feature`
5.  **Abre un Pull Request**: Cuéntanos qué has mejorado y lo revisaremos con gusto.

¿Encontraste un bug? 🐛 Abre un issue. ¿Tienes una idea? 💡 Compártela.

¡Construyamos el mejor ecosistema juntos!

---
¡Únete a la revolución de **Umbral** y construye el futuro hoy! ✨
