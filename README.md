# 🏦 Vault Distributor

[![Stellar](https://img.shields.io/badge/Stellar-Soroban-7D00FF?style=flat&logo=stellar)](https://stellar.org)
[![Rust](https://img.shields.io/badge/Rust-1.91+-orange?style=flat&logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-passing-brightgreen.svg)](contracts/vault-distributor/src/test.rs)

> Smart contract en Stellar (Soroban) para distribución automatizada de fondos desde un vault administrado por una wallet maestra.

## 📋 Descripción

Vault Distributor es un contrato inteligente desarrollado en Rust para la blockchain de Stellar que permite la distribución equitativa de fondos a múltiples destinatarios desde un vault centralizado. El contrato está protegido por un sistema de administración que requiere autenticación de una wallet maestra registrada.

### ✨ Características Principales

- 🔐 **Administración segura**: Wallet admin única con autenticación requerida
- 💸 **Distribución equitativa**: Reparto automático de fondos entre destinatarios
- ✅ **Validaciones robustas**: Verificación de parámetros y permisos
- 📊 **Eventos auditables**: Registro de operaciones para trazabilidad
- 🧪 **Tests completos**: Suite de pruebas unitarias validadas
- ⚡ **Optimizado**: Compilado a WebAssembly para máxima eficiencia

## 🏗️ Estructura del Proyecto

```text
refi-universe-sc/
├── contracts/
│   └── vault-distributor/
│       ├── src/
│       │   ├── lib.rs          # Lógica principal del contrato
│       │   └── test.rs         # Tests unitarios
│       ├── Cargo.toml          # Dependencias del contrato
│       └── Makefile            # Comandos de compilación
├── Cargo.toml                  # Workspace configuration
├── BACKLOG.md                  # Casos de uso y progreso
├── .env.example                # Variables de entorno template
└── README.md
```

## 🚀 Inicio Rápido

### Prerequisitos

- [Rust](https://www.rust-lang.org/tools/install) >= 1.91
- [Stellar CLI](https://developers.stellar.org/docs/tools/cli) >= 23.2.1
- Target WebAssembly: `wasm32v1-none`

### Instalación

```bash
# 1. Clonar el repositorio
git clone https://github.com/DevCristobalvc/refi-universe-sc.git
cd refi-universe-sc

# 2. Instalar Rust (si no lo tienes)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 3. Instalar Stellar CLI
brew install stellar-cli

# 4. Agregar target WebAssembly
rustup target add wasm32v1-none

# 5. Configurar variables de entorno
cp .env.example .env
# Editar .env con tus credenciales
```

### Compilación

```bash
# Ejecutar tests
cd contracts/vault-distributor
cargo test

# Compilar a WebAssembly
cargo build --target wasm32v1-none --release

# El archivo .wasm estará en:
# target/wasm32v1-none/release/vault_distributor.wasm
```

## 📖 Uso

### Configurar Wallet Admin

```bash
# Generar keypair y fondear en testnet
stellar keys generate admin --network testnet --fund

# Ver dirección pública
stellar keys address admin
```

### Desplegar Contrato

```bash
# Desplegar a testnet
stellar contract deploy \
  --wasm target/wasm32v1-none/release/vault_distributor.wasm \
  --source-account admin \
  --network testnet \
  --alias vault_distributor
```

### Inicializar Contrato

```bash
# Registrar wallet admin
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account admin \
  --network testnet \
  -- init \
  --admin <ADMIN_PUBLIC_KEY>
```

### Consultar Admin

```bash
# Obtener admin registrado
stellar contract invoke \
  --id <CONTRACT_ID> \
  --network testnet \
  -- get_admin
```

## 🧪 Testing

```bash
cd contracts/vault-distributor
cargo test

# Output esperado:
# running 4 tests
# test test::test_init_success ... ok
# test test::test_get_admin_not_found - should panic ... ok
# test test::test_init_already_initialized - should panic ... ok
# test test::test_init_requires_auth ... ok
```

## 📚 API del Contrato

### `init(admin: Address)`

Inicializa el contrato con una wallet admin.

- **Parámetros**: 
  - `admin`: Dirección de la wallet maestra
- **Requiere**: Autenticación del admin
- **Solo**: Se ejecuta una vez

### `get_admin() -> Address`

Retorna la dirección del admin registrado.

- **Retorna**: Dirección de la wallet admin
- **No requiere**: Autenticación

## 🗺️ Roadmap

Ver [BACKLOG.md](BACKLOG.md) para el detalle completo de casos de uso.

- [x] Inicializar contrato (init)
- [x] Obtener admin (get_admin)
- [ ] Validar admin en distribución
- [ ] Validar parámetros de distribución
- [ ] Cálculo de distribución
- [ ] Enviar pagos desde el contrato
- [ ] Emitir eventos
- [ ] Manejo de errores
- [ ] Despliegue del contrato
- [ ] Pruebas unitarias completas
- [ ] Auditoría y trazabilidad

**Progreso**: 16.67% (2/12 casos de uso)

## 🔧 Configuración

### Variables de Entorno

```bash
# .env
STELLAR_NETWORK=testnet
ADMIN_SECRET_KEY=S...
ADMIN_PUBLIC_KEY=G...
CONTRACT_ID=C...
STELLAR_RPC_URL=https://soroban-testnet.stellar.org
SOURCE_ACCOUNT_NAME=admin
USE_FRIENDBOT=true
DEBUG=true
```

## 🤝 Contribuir

Las contribuciones son bienvenidas. Por favor:

1. Fork el proyecto
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'feat: add amazing feature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

## 📄 Licencia

Este proyecto está bajo la Licencia MIT. Ver archivo `LICENSE` para más detalles.

## 🔗 Enlaces

- [Documentación Stellar](https://developers.stellar.org)
- [Soroban Docs](https://developers.stellar.org/docs/smart-contracts)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Stellar CLI](https://developers.stellar.org/docs/tools/cli)

## 👥 Autores

- [@DevCristobalvc](https://github.com/DevCristobalvc)

---

**Nota**: Este proyecto está en desarrollo activo. Para producción, asegúrate de realizar auditorías de seguridad completas.
