# winevtx

[![PyPI version](https://img.shields.io/pypi/v/winevtx.svg)](https://pypi.org/project/winevtx/)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE.md)

Librería de alto rendimiento para Python construida en **Rust** con **PyO3** y **Maturin**, diseñada para:
1. **Consultar eventos del sistema operativo en vivo** utilizando la API nativa de Windows (`wevtapi.dll` / `EvtQuery` / `EvtRender`).
2. **Parsear y analizar archivos `.evtx` offline** con soporte multi-hilo acelerado por Rayon.
3. Devolver datos estructurados de forma ultrarrápida como objetos `EventRecord`, diccionarios nativos (`dict`), cadenas `XML` crudas o `JSON`.

---

## Características

- ⚡ **Rendimiento nativo**: Motor en Rust con cero sobrecoste y liberación del GIL durante consultas intensivas de I/O y decodificación.
- 🔴 **Eventos en vivo del OS**: Acceso a canales como `System`, `Security`, `Application` o cualquier canal registrado, con filtros XPath personalizados (ej. `*[System[(Level <= 3)]]`).
- 📁 **Archivos `.evtx` offline**: Lectura forense de registros exportados de Windows sin depender de utilidades externas.
- 🔄 **Múltiples formatos**:
  - `record` (por defecto): Objeto `EventRecord` con propiedades directas (`event_id`, `provider`, `channel`, `time_created`, `event_data`, `system`), método `.to_dict()` y soporte para indexación `ev["System"]`.
  - `dict`: Diccionario nativo de Python deserializado directamente en CPython.
  - `xml`: Cadena XML nativa de Windows.
  - `json`: Cadena JSON formateada.
- 📦 **Type hints completos**: Soporte estático PEP 561 (`py.typed`) y stubs tipados para autocompletado instantáneo en VS Code, PyCharm, etc.

---

## Instalación y Compilación

Para compilar y probar en desarrollo local con Maturin:

```bash
# Crear entorno virtual e instalar maturin
python -m venv .venv
.\.venv\Scripts\pip install maturin

# Compilar e instalar en modo editable
.\.venv\Scripts\maturin develop
```

O generar una rueda (.whl):

```bash
maturin build --release
```

---

## Guía de Uso

### 1. Leer eventos del OS en vivo

#### Consulta rápida de eventos recientes
```python
import winevtx

# Obtener los 10 eventos más recientes del canal System
events = winevtx.query_events(channel="System", limit=10)

for ev in events:
    print(f"[{ev.time_created}] EventID: {ev.event_id} | Provider: {ev.provider}")
    print(ev.to_dict())
```

#### Filtros con XPath queries
```python
import winevtx

# Filtrar solo eventos de error y advertencia (Level <= 3)
query = "*[System[(Level <= 3)]]"
events = winevtx.query_events(channel="System", query=query, limit=5)

for ev in events:
    print(f"Alerta nivel {ev.level}: {ev.provider} (ID: {ev.event_id})")
```

#### Iterador en streaming
```python
import winevtx

# Stream de eventos sin cargar todos en memoria
for ev in winevtx.iter_events(channel="Application", reverse=True):
    if ev.event_id == 1000:
        print("Application Error detectado:", ev.event_data)
        break
```

---

### 2. Leer archivos `.evtx` offline

```python
import winevtx

# Leer directamente un archivo .evtx
events = winevtx.read_evtx(r"C:\Logs\Security.evtx", limit=50)

# O como context manager
with winevtx.EvtxFile(r"C:\Logs\System.evtx") as f:
    for ev in f.iter(format="record"):
        print(ev.event_id, ev.computer, ev.time_created)
```

---

### 3. Formatos de salida (`record`, `dict`, `xml`, `json`)

```python
import winevtx

# 1. Objeto EventRecord (por defecto)
ev = winevtx.query_events("System", limit=1, format="record")[0]
print(ev.event_id)
print(ev.xml)           # XML crudo
print(ev.to_dict())     # Dict de Python
print(ev.to_json())     # JSON string

# 2. Diccionario de Python puro
data = winevtx.query_events("System", limit=1, format="dict")[0]
print(data["Event"]["System"]["Channel"])

# 3. XML nativo en crudo
raw_xml = winevtx.query_events("System", limit=1, format="xml")[0]
print(raw_xml)

# 4. JSON
json_str = winevtx.query_events("System", limit=1, format="json")[0]
print(json_str)
```

---

### 4. Utilidades auxiliares

```python
import winevtx

xml_string = "<Event>...</Event>"

# Convertir cualquier XML de evento de Windows a dict
d = winevtx.xml_to_dict(xml_string)

# Convertir XML a JSON
j = winevtx.xml_to_json(xml_string)
```

---

## Referencia de API

### Funciones Principales

- `query_events(channel="System", query="*", limit=None, reverse=True, format="record") -> List`: Consulta eventos en vivo de un canal de Windows.
- `iter_events(channel="System", query="*", reverse=True, format="record") -> LiveEventIterator`: Iterador sobre eventos en vivo.
- `read_evtx(path, limit=None, format="record") -> List`: Lee eventos de un archivo `.evtx` en disco.
- `iter_evtx(path, format="record") -> EvtxRecordIterator`: Iterador sobre un archivo `.evtx`.
- `xml_to_dict(xml: str) -> dict`: Parsea XML de evento a diccionario de Python.
- `xml_to_json(xml: str) -> str`: Parsea XML de evento a cadena JSON.

### Clases

- `winevtx.EventRecord`: Contenedor del evento.
  - Atributos: `xml`, `event_id`, `record_id`, `channel`, `provider`, `level`, `time_created`, `computer`, `system`, `event_data`.
  - Métodos: `to_dict()`, `to_json()`, `get(key, default)`.
- `winevtx.LiveEventLog(channel, query, reverse)`: Manejador de canal en vivo.
- `winevtx.EvtxFile(path)`: Manejador de archivo `.evtx` offline (con soporte `with`).

---

## Licencia

Este proyecto está bajo la Licencia MIT.
