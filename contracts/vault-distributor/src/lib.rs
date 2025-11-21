#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, symbol_short, token, Address, Env, Vec};

/// Storage key for the admin address
#[contracttype]
pub enum StorageKey {
    Admin,
}

/// Custom errors for the contract
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// Admin already initialized
    AlreadyInitialized = 1,
    /// Admin not found
    AdminNotFound = 2,
    /// Unauthorized: caller is not admin
    Unauthorized = 3,
    /// Recipients list is empty
    EmptyRecipients = 4,
    /// Total amount must be positive
    InvalidAmount = 5,
    /// Amount per recipient is zero
    ZeroAmountEach = 6,
}

#[contract]
pub struct VaultDistributor;

#[contractimpl]
impl VaultDistributor {
    /// Inicializar el contrato con una wallet admin
    /// 
    /// # Arguments
    /// * `env` - El entorno del contrato
    /// * `admin` - La dirección de la wallet que será el admin
    /// 
    /// # Criterios:
    /// - Debe recibir una dirección de wallet válida
    /// - La wallet debe firmar la transacción (require_auth)
    /// - Se almacena en storage instance bajo la clave Admin
    /// - Solo puede ejecutarse correctamente una vez
    pub fn init(env: Env, admin: Address) {
        // Verificar que el admin no esté ya inicializado
        // Si ya existe, panic con el error
        if env.storage().instance().has(&StorageKey::Admin) {
            panic!("Admin already initialized");
        }

        // Requerir autenticación del admin
        admin.require_auth();

        // Almacenar el admin en storage instance
        env.storage().instance().set(&StorageKey::Admin, &admin);
    }

    /// Obtener la dirección del admin registrado
    /// 
    /// # Arguments
    /// * `env` - El entorno del contrato
    /// 
    /// # Returns
    /// * `Address` - La dirección del admin
    /// 
    /// # Criterios:
    /// - Retorna la dirección guardada en storage
    /// - No requiere autenticación
    /// - Falla si no existe admin (panic)
    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&StorageKey::Admin)
            .unwrap_or_else(|| panic!("Admin not found"))
    }

    /// Distribuir fondos equitativamente a múltiples destinatarios
    /// 
    /// # Arguments
    /// * `env` - El entorno del contrato
    /// * `token` - La dirección del token a distribuir
    /// * `recipients` - Vector de direcciones destinatarias
    /// * `total_amount` - Monto total a distribuir
    /// 
    /// # Criterios:
    /// - Solo el admin puede ejecutar esta función
    /// - recipients debe contener al menos 1 wallet
    /// - total_amount debe ser positivo
    /// - amount_each debe ser mayor que 0
    /// - Se envía el mismo monto a cada destinatario
    /// - Se emite un evento al finalizar
    pub fn distribute(
        env: Env,
        token: Address,
        recipients: Vec<Address>,
        total_amount: i128,
    ) {
        // 1. Validar que el caller es el admin
        let admin = Self::get_admin(env.clone());
        admin.require_auth();

        // 2. Validar parámetros de entrada
        // Verificar que hay al menos un destinatario
        if recipients.is_empty() {
            panic!("Recipients list cannot be empty");
        }

        // Verificar que el monto total es positivo
        if total_amount <= 0 {
            panic!("Total amount must be positive");
        }

        // 3. Calcular monto por destinatario
        let num_recipients = recipients.len() as i128;
        let amount_each = total_amount / num_recipients;

        // Verificar que el monto por destinatario es mayor que 0
        if amount_each <= 0 {
            panic!("Amount per recipient must be greater than zero");
        }

        // 4. Obtener el cliente del token
        let token_client = token::Client::new(&env, &token);

        // 5. Enviar pagos a cada destinatario
        for recipient in recipients.iter() {
            token_client.transfer(
                &env.current_contract_address(),
                &recipient,
                &amount_each,
            );
        }

        // 6. Emitir evento de distribución exitosa
        env.events().publish(
            (symbol_short!("vault"), symbol_short!("distribtd")),
            (total_amount, num_recipients),
        );
    }
}

mod test;
