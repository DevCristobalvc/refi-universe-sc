# BACKLOG DEL PROYECTO "VAULT DISTRIBUTOR"

## 🔧 Estado del Proyecto
- [ ] En desarrollo
- [ ] Listo para deploy
- [ ] Desplegado en testnet
- [ ] Desplegado en mainnet

---

## 📋 Casos de Uso

### ✅ Caso de uso: Inicializar contrato (init)
**Estado:** ✅ Completado

**Descripción:** El sistema debe permitir registrar una única wallet maestra (admin) al momento de desplegar el contrato.

**Criterios de aceptación:**
- [x] Debe recibirse como parámetro una dirección de wallet válida
- [x] La wallet enviada debe firmar la transacción (require_auth)
- [x] El valor debe almacenarse en storage instance bajo la clave admin
- [x] Si la wallet no firma, la transacción falla
- [x] Solo puede ejecutarse correctamente una vez; llamadas posteriores deben fallar o sobrescribir solo si se define explícitamente
- [x] El contrato debe quedar listo para permitir operaciones posteriores de administración y distribución

---

### ✅ Caso de uso: Obtener admin (get_admin)
**Estado:** ✅ Completado

**Descripción:** Permitir consultar cuál es la wallet maestra registrada.

**Criterios de aceptación:**
- [x] Debe retornar siempre la dirección guardada en el storage
- [x] No debe requerir autenticación para consultar
- [x] Si no existe admin en storage, debe fallar
- [x] Debe funcionar en cualquier momento después de init

---

### ✅ Caso de uso: Validar admin en distribución
**Estado:** ⏳ Pendiente

**Descripción:** Antes de distribuir fondos, se debe verificar que la wallet que invoca es el admin registrado.

**Criterios de aceptación:**
- [ ] La dirección usada para firmar la transacción debe coincidir exactamente con la dirección almacenada
- [ ] Si la firma no coincide, la función debe abortar con error
- [ ] Debe ejecutarse require_auth() sobre la wallet del admin
- [ ] No se puede continuar con distribución si no se cumple la autenticación

---

### ✅ Caso de uso: Validar parámetros de distribución
**Estado:** ⏳ Pendiente

**Descripción:** Verificar que la entrada del usuario es válida antes de transferir fondos.

**Criterios de aceptación:**
- [ ] recipients debe contener al menos 1 wallet
- [ ] total_amount debe ser un entero positivo
- [ ] El monto total debe ser divisible en partes iguales por la cantidad de destinatarios
- [ ] amount_each debe ser mayor que 0; si no, debe abortar
- [ ] Si recipients está vacío, la llamada debe fallar inmediatamente
- [ ] Si total_amount es menor que la cantidad de destinatarios, la transacción falla

---

### ✅ Caso de uso: Cálculo de distribución
**Estado:** ⏳ Pendiente

**Descripción:** Calcular con precisión cuánto recibe cada wallet.

**Criterios de aceptación:**
- [ ] amount_each = total_amount / len(recipients)
- [ ] El cálculo debe ser exacto usando tipo i128
- [ ] No debe haber truncamiento no deseado: si la división deja residuo, el sistema debe aceptarlo siempre que amount_each sea mayor a 0
- [ ] Debe registrarse internamente el valor calculado por si se desea auditar

---

### ✅ Caso de uso: Enviar pagos desde el contrato
**Estado:** ⏳ Pendiente

**Descripción:** Ejecución efectiva de los envíos a cada wallet desde el baúl del contrato.

**Criterios de aceptación:**
- [ ] El envío debe usar env.payments().pay(destinatario, amount_each)
- [ ] Cada wallet recibe exactamente el valor calculado
- [ ] La transacción debe enviar todos los pagos dentro de la misma operación
- [ ] Si algún pago falla, toda la operación debe revertir
- [ ] El contrato solo envía fondos que ya tiene en su balance
- [ ] No debe enviarse más de lo permitido por total_amount

---

### ✅ Caso de uso: Emitir eventos
**Estado:** ⏳ Pendiente

**Descripción:** Registrar un evento posterior a la distribución para auditoría.

**Criterios de aceptación:**
- [ ] Debe emitirse un evento bajo las claves ("vault", "distributed")
- [ ] El evento debe incluir el total_amount
- [ ] El evento debe generarse solo cuando la distribución termina correctamente
- [ ] Si ocurre un error previo, no se debe emitir evento

---

### ✅ Caso de uso: Manejo de errores
**Estado:** ⏳ Pendiente

**Descripción:** Definir comportamientos claros ante entradas inválidas o falta de permisos.

**Criterios de aceptación:**
- [ ] Error si recipients es 0
- [ ] Error si total_amount <= 0
- [ ] Error si amount_each <= 0
- [ ] Error si la firma no pertenece al admin
- [ ] Error si el contrato no tiene fondos suficientes para completar la distribución
- [ ] Error si no existe admin configurado

---

### ✅ Caso de uso: Despliegue del contrato
**Estado:** ⏳ Pendiente

**Descripción:** Procedimiento para subir el contrato a la red y dejarlo operativo.

**Criterios de aceptación:**
- [ ] Debe compilarse en modo no_std
- [ ] Debe desplegarse con el payload correcto que incluya las funciones init, get_admin y distribute
- [ ] Debe ejecutarse init inmediatamente después del deploy
- [ ] La wallet que ejecuta init será la única wallet maestra

---

### ✅ Caso de uso: Pruebas unitarias
**Estado:** ⏳ Pendiente

**Descripción:** Crear pruebas automatizadas para validar el comportamiento del contrato.

**Criterios de aceptación:**
- [ ] Prueba: init registra correctamente el admin
- [ ] Prueba: get_admin devuelve el valor correcto
- [ ] Prueba: distribute falla si no es llamada por admin
- [ ] Prueba: distribute falla si recipients está vacío
- [ ] Prueba: distribute falla si el monto total no permite amount_each > 0
- [ ] Prueba: distribute envía correctamente los pagos
- [ ] Prueba: se emite el evento esperado después de distribuir

---

### ✅ Caso de uso: Auditoría y trazabilidad interna
**Estado:** ⏳ Pendiente

**Descripción:** Asegurar que el uso del contrato sea auditable.

**Criterios de aceptación:**
- [ ] Los eventos deben registrar información mínima necesaria para reconstruir las operaciones
- [ ] Debe ser posible consultar todas las direcciones que recibieron fondos mediante los logs
- [ ] Las validaciones y errores deben mostrarse de forma clara y predecible

---

## 📝 Notas de Desarrollo

### Stack Tecnológico
- **Lenguaje:** Rust
- **Plataforma:** Stellar (Soroban)
- **Target:** wasm32v1-none
- **SDK:** Soroban Rust SDK

### Comandos Útiles
```bash
# Compilar contrato
cargo build --target wasm32v1-none --release

# Desplegar a testnet
stellar contract deploy \
  --wasm target/wasm32v1-none/release/vault_distributor.wasm \
  --source-account admin \
  --network testnet

# Ejecutar tests
cargo test

# Invocar función init
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account admin \
  --network testnet \
  -- init --admin <ADMIN_ADDRESS>
```

---

## 🎯 Progreso General
- **Total de casos de uso:** 12
- **Completados:** 2
- **En progreso:** 0
- **Pendientes:** 10
- **Progreso:** 16.67%

---

**Última actualización:** 21 de noviembre de 2025
