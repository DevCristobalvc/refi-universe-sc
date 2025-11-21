#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_init_success() {
    let env = Env::default();
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);

    // Crear una dirección de admin de prueba
    let admin = Address::generate(&env);

    // Mock de la autenticación
    env.mock_all_auths();

    // Inicializar el contrato
    client.init(&admin);

    // Verificar que el admin fue almacenado correctamente
    let stored_admin = client.get_admin();
    assert_eq!(stored_admin, admin);
}

#[test]
#[should_panic(expected = "Admin already initialized")]
fn test_init_already_initialized() {
    let env = Env::default();
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    env.mock_all_auths();

    // Primera inicialización - debe funcionar
    client.init(&admin);

    // Segunda inicialización - debe fallar con panic
    let admin2 = Address::generate(&env);
    client.init(&admin2);
}

#[test]
#[should_panic(expected = "Admin not found")]
fn test_get_admin_not_found() {
    let env = Env::default();
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);

    // Intentar obtener admin sin inicializar - debe fallar con panic
    client.get_admin();
}

#[test]
fn test_init_requires_auth() {
    let env = Env::default();
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    // Mockear todas las autenticaciones
    env.mock_all_auths();
    
    client.init(&admin);
    
    // Verificar que se requirió autenticación del admin
    use soroban_sdk::{IntoVal, symbol_short};
    assert_eq!(
        env.auths(),
        std::vec![(
            admin.clone(),
            soroban_sdk::testutils::AuthorizedInvocation {
                function: soroban_sdk::testutils::AuthorizedFunction::Contract((
                    contract_id.clone(),
                    symbol_short!("init"),
                    (admin.clone(),).into_val(&env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
}
