use fuels::{prelude::*, tx::ContractId};
use fuels_abigen_macro::abigen;


// Load abi from json
abigen!(MyContract, "out/debug/simple_example_counter-abi.json");

async fn get_contract_instance() -> (MyContract, ContractId) {
    // Launch a local network and deploy the contract
    let wallet = launch_provider_and_get_single_wallet().await;

    let id = Contract::deploy("./out/debug/simple_example_counter.bin", &wallet, TxParameters::default())
        .await
        .unwrap();

    let instance = MyContract::new(id.to_string(), wallet);

    (instance, id)
}

#[tokio::test]
async fn can_get_contract_id() {
    let (_instance, _id) = get_contract_instance().await;
}

#[tokio::test]
async fn can_initialize_storage() {
    let (_instance, _) = get_contract_instance().await;
    let value: u64 = 42;
    let counter_val = _instance.initialize_counter(value).call().await.unwrap();
    assert_eq!(counter_val.value, value);
}

#[tokio::test]
async fn can_increment_counter() {
    let (_instance, _) = get_contract_instance().await;
    let initial_value = 80;
    let increment = 1;
    _instance.initialize_counter(initial_value).call().await.unwrap();
    let updated_val = _instance.increment_counter(increment).call().await.unwrap();
    assert_eq!(updated_val.value, initial_value + increment);
}