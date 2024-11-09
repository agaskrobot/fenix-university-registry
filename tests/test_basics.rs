use fenix_university_registry::University;
use serde_json::json;

#[tokio::test]
async fn test_contract_is_operational() -> Result<(), Box<dyn std::error::Error>> {
    let contract_wasm = near_workspaces::compile_project("./").await?;

    test_basics_on(&contract_wasm).await?;
    Ok(())
}

async fn test_basics_on(contract_wasm: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract = sandbox.dev_deploy(contract_wasm).await?;

    let outcome = contract
        .call("add_university")
        .args_json(json!({"name": "UMA", "account_id": "admin"}))
        .transact()
        .await?;
    assert!(outcome.is_success());

    let university_json = contract
    .view("get_university_by_account_id")
    .args_json(json!({"account_id": "admin"}))
    .await?
    .json::<University>()?;

    let expected_university = University {
        name: "UMA".to_string(),
        account_id: "admin".to_string(),
    };

    assert_eq!(expected_university, university_json);
    
    Ok(())
}
