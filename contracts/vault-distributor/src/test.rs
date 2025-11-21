#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    token, vec, Address, Env, IntoVal,
};

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
    use soroban_sdk::symbol_short;
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

// ============= Tests para distribute =============

#[test]
fn test_distribute_success() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);

    // Inicializar contrato
    client.init(&admin);

    // Crear token de prueba
    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_id = token_contract.address();
    let token_client = token::StellarAssetClient::new(&env, &token_id);

    // Mintear tokens al contrato
    token_client.mint(&contract_id, &1000);

    // Crear destinatarios
    let recipient1 = Address::generate(&env);
    let recipient2 = Address::generate(&env);
    let recipient3 = Address::generate(&env);
    let recipients = vec![&env, recipient1.clone(), recipient2.clone(), recipient3.clone()];

    // Distribuir 900 tokens (300 cada uno)
    client.distribute(&token_id, &recipients, &900);

    // Verificar balances - cada uno recibe 300 (900 / 3)
    assert_eq!(token_client.balance(&recipient1), 300);
    assert_eq!(token_client.balance(&recipient2), 300);
    assert_eq!(token_client.balance(&recipient3), 300);
    
    // Verificar que el contrato tiene el sobrante
    assert_eq!(token_client.balance(&contract_id), 100);
}

#[test]
#[should_panic(expected = "Recipients list cannot be empty")]
fn test_distribute_empty_recipients() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);

    client.init(&admin);

    let token_id = Address::generate(&env);
    let recipients = vec![&env]; // Lista vacía

    // Debe fallar
    client.distribute(&token_id, &recipients, &1000);
}

#[test]
#[should_panic(expected = "Total amount must be positive")]
fn test_distribute_invalid_amount() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);

    client.init(&admin);

    let token_id = Address::generate(&env);
    let recipient = Address::generate(&env);
    let recipients = vec![&env, recipient];

    // Debe fallar con monto 0
    client.distribute(&token_id, &recipients, &0);
}

#[test]
#[should_panic(expected = "Total amount must be positive")]
fn test_distribute_negative_amount() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);

    client.init(&admin);

    let token_id = Address::generate(&env);
    let recipient = Address::generate(&env);
    let recipients = vec![&env, recipient];

    // Debe fallar con monto negativo
    client.distribute(&token_id, &recipients, &-100);
}

#[test]
#[should_panic(expected = "Amount per recipient must be greater than zero")]
fn test_distribute_amount_too_small() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);

    client.init(&admin);

    let token_id = Address::generate(&env);
    let recipient1 = Address::generate(&env);
    let recipient2 = Address::generate(&env);
    let recipient3 = Address::generate(&env);
    let recipients = vec![&env, recipient1, recipient2, recipient3];

    // 2 / 3 = 0, debe fallar
    client.distribute(&token_id, &recipients, &2);
}

#[test]
fn test_distribute_requires_admin_auth() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);

    client.init(&admin);

    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_id = token_contract.address();
    let token_client = token::StellarAssetClient::new(&env, &token_id);
    token_client.mint(&contract_id, &1000);

    let recipient = Address::generate(&env);
    let recipients = vec![&env, recipient];

    client.distribute(&token_id, &recipients, &500);

    // Verificar que el admin fue autenticado
    let auths = env.auths();
    let admin_auth = auths.iter().find(|(addr, _)| addr == &admin);
    assert!(admin_auth.is_some());
}
