use axum::{extract::State, response::Json, routing::get, serve, Router};
use reqwest::Client;
use serde_json::Value;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;
use tracing::info;

#[derive(Clone)]
struct AppState {
    client: Client,
    rpc_url: String,
    contract_id: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let rpc_url = env::var("STELLAR_RPC_URL").unwrap_or_else(|_| "https://rpc-futurenet.stellar.org".to_string());
    let contract_id = env::var("VALENCE_CONTRACT_ID").unwrap_or_else(|_| "replace-with-contract-id".to_string());

    let state = AppState {
        client: Client::new(),
        rpc_url,
        contract_id,
    };

    let app = Router::new()
        .route("/capacity", get(get_capacity))
        .route("/nodes", get(get_nodes))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!(?addr, "Valence backend starting");

    let listener = TcpListener::bind(addr).await?;
    serve(listener, app).await?;

    Ok(())
}

#[axum::debug_handler]
async fn get_capacity(State(state): State<AppState>) -> Json<Value> {
    let summary = query_capacity(&state).await.unwrap_or_else(|_| serde_json::json!({"total_cpu_cores":0,"total_gpu_units":0,"total_storage_gb":0}));
    Json(summary)
}

#[axum::debug_handler]
async fn get_nodes(State(state): State<AppState>) -> Json<Vec<Value>> {
    let nodes = query_node_list(&state).await.unwrap_or_default();
    Json(nodes)
}

async fn query_capacity(state: &AppState) -> anyhow::Result<Value> {
    let body = serde_json::json!({
        "contract_id": state.contract_id,
        "key": { "type": "symbol", "value": "global_capacity" }
    });

    let response: Value = state
        .client
        .post(format!("{}/soroban/contract/data", state.rpc_url.trim_end_matches('/')))
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(serde_json::json!({
        "total_cpu_cores": response["cpu"].as_u64().unwrap_or(0) as u32,
        "total_gpu_units": response["gpu"].as_u64().unwrap_or(0) as u32,
        "total_storage_gb": response["storage"].as_u64().unwrap_or(0) as u32,
    }))
}

async fn query_node_list(state: &AppState) -> anyhow::Result<Vec<Value>> {
    let body = serde_json::json!({
        "contract_id": state.contract_id,
        "key": { "type": "symbol", "value": "registered_operators" }
    });

    let response: Value = state
        .client
        .post(format!("{}/soroban/contract/data", state.rpc_url.trim_end_matches('/')))
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let operators = response["value"].as_array().unwrap_or(&Vec::new()).iter().filter_map(|item| item.as_str().map(str::to_owned)).collect::<Vec<_>>();

    let mut records = Vec::new();
    for operator in operators {
        if let Ok(record) = query_node_record(state, &operator).await {
            records.push(record);
        }
    }

    Ok(records)
}

async fn query_node_record(state: &AppState, operator: &str) -> anyhow::Result<Value> {
    let body = serde_json::json!({
        "contract_id": state.contract_id,
        "key": {
            "type": "map",
            "value": {
                "operator": operator,
                "field": "node_record"
            }
        }
    });

    let response: Value = state
        .client
        .post(format!("{}/soroban/contract/data", state.rpc_url.trim_end_matches('/')))
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(serde_json::json!({
        "operator": operator,
        "config": {
            "cpu_cores": response["config"]["cpu_cores"].as_u64().unwrap_or(0) as u32,
            "gpu_units": response["config"]["gpu_units"].as_u64().unwrap_or(0) as u32,
            "storage_gb": response["config"]["storage_gb"].as_u64().unwrap_or(0) as u32,
        },
        "stake": response["stake"].as_i64().unwrap_or(0) as i128,
        "active": response["active"].as_bool().unwrap_or(false),
        "reward_points": response["reward_points"].as_u64().unwrap_or(0) as u32,
    }))
}
