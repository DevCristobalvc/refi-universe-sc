#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env};

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
}

mod test;
