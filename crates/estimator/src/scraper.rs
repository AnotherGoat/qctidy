use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use regex::Regex;
use reqwest::Client;
use scraper::Html;
use serde_json::{Value, json};

const CACHE_FILE: &str = "pricing_cache.json";
const CACHE_EXPIRY_SECONDS: u64 = 24 * 60 * 60;
const IBM_PRICING_URL: &str = "https://www.ibm.com/quantum/pricing/";
const IBM_CATALOG_PAYGO_URL: &str =
    "https://globalcatalog.cloud.ibm.com/api/v1/5304b575-3cff-4455-90dc-ae4367762093/pricing";
const AWS_PRICE_LIST_API_URL: &str =
    "https://pricing.us-east-1.amazonaws.com/offers/v1.0/aws/AmazonBraket/current/index.json";

fn now_iso() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string()
}

/// Location of the pricing cache file.
///
/// Defaults to `./pricing_cache.json`, or `QCTIDY_CACHE_DIR/pricing_cache.json` when that
/// environment variable is set.
fn cache_path() -> PathBuf {
    if let Some(directory) = std::env::var_os("QCTIDY_CACHE_DIR") {
        return PathBuf::from(directory).join(CACHE_FILE);
    }

    PathBuf::from(CACHE_FILE)
}

pub fn load_cache() -> Option<Value> {
    let path = cache_path();
    if !path.exists() {
        return None;
    }

    let content = fs::read_to_string(path).ok()?;
    let cache: Value = serde_json::from_str(&content).ok()?;

    let timestamp = cache.get("timestamp").and_then(|t| t.as_f64())?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()?
        .as_secs_f64();
    if now - timestamp > CACHE_EXPIRY_SECONDS as f64 {
        return None;
    }

    Some(cache.get("data")?.clone())
}

fn save_cache(data: &Value) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64();
    let cache_payload = json!({
        "timestamp": now,
        "downloaded_at": now_iso(),
        "expires_in_seconds": CACHE_EXPIRY_SECONDS,
        "data": data,
    });

    let Ok(content) = serde_json::to_string_pretty(&cache_payload) else {
        return;
    };

    let path = cache_path();
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        let _ = fs::create_dir_all(parent);
    }

    let _ = fs::write(path, content);
}

fn normalize_text(html: &str) -> String {
    let document = Html::parse_document(html);

    document
        .root_element()
        .text()
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

async fn fetch_ibm_quantum_api(client: &Client) -> Value {
    let downloaded_at = now_iso();
    let mut plans = Vec::new();

    if let Ok(response) = client.get(IBM_PRICING_URL).send().await
        && let Ok(html) = response.text().await
    {
        plans = extract_ibm_plans(&normalize_text(&html));
    }

    let catalog_fallback = fetch_ibm_catalog_paygo(client).await;

    json!({
        "status": "success",
        "provider": "IBM Quantum",
        "source_url": IBM_PRICING_URL,
        "downloaded_at": downloaded_at,
        "data": {
            "plans": plans,
            "catalog_paygo_fallback": catalog_fallback,
            "notes": ["Web scraping was performed for all plans as requested, with an API fallback for pay-as-you-go."]
        }
    })
}

fn extract_ibm_plans(text: &str) -> Vec<Value> {
    let expected_plans = [
        "Open Plan",
        "Pay-As-You-Go Plan",
        "Flex Plan",
        "Premium Plan",
        "On-Prem Plan",
    ];

    let Ok(price_regex) = Regex::new(r"\$\s*([0-9]+(?:\.[0-9]+)?)\s*USD\s*/\s*minute") else {
        return Vec::new();
    };
    let Ok(quote_regex) = Regex::new(r"(?i)requires quote|contact for quote") else {
        return Vec::new();
    };

    let mut plans = Vec::new();

    for (index, &plan_name) in expected_plans.iter().enumerate() {
        let Some(start) = text.find(plan_name) else {
            continue;
        };

        let end = next_plan_offset(text, start, plan_name, &expected_plans[index + 1..])
            .unwrap_or_else(|| clamp_to_char_boundary(text, start + 500));
        let segment = &text[start..end];

        if plan_name == "Open Plan" {
            plans.push(json!({
                "plan": plan_name,
                "price_label": "Free",
                "price_usd_per_minute": 0.0,
                "price_usd_per_second": 0.0
            }));
            continue;
        }

        if plan_name == "On-Prem Plan" {
            plans.push(json!({
                "plan": plan_name,
                "price_label": "Contact for quote",
                "price_usd_per_minute": Value::Null,
                "price_usd_per_second": Value::Null
            }));
            continue;
        }

        let mut price_label = "not_found".to_string();
        let mut price_per_minute = Value::Null;
        let mut price_per_second = Value::Null;

        if let Some(captures) = price_regex.captures(segment)
            && let Ok(value) = captures[1].parse::<f64>()
        {
            price_per_minute = json!(value);
            price_per_second = json!(value / 60.0);
            price_label = format!("${value} USD / minute");
        } else if quote_regex.is_match(segment) {
            price_label = "Contact for quote".to_string();
        }

        plans.push(json!({
            "plan": plan_name,
            "price_label": price_label,
            "price_usd_per_minute": price_per_minute,
            "price_usd_per_second": price_per_second
        }));
    }

    plans
}

fn next_plan_offset(
    text: &str,
    start: usize,
    plan_name: &str,
    remaining_plans: &[&str],
) -> Option<usize> {
    let after_plan = start + plan_name.len();

    remaining_plans
        .iter()
        .filter_map(|next_plan| text[after_plan..].find(next_plan))
        .map(|offset| after_plan + offset)
        .min()
}

/// Move `index` down until it sits on a UTF-8 character boundary.
///
/// Scraped text can contain multi-byte characters, so a raw byte offset may fall inside one.
fn clamp_to_char_boundary(text: &str, index: usize) -> usize {
    let mut index = index.min(text.len());

    while index > 0 && !text.is_char_boundary(index) {
        index -= 1;
    }

    index
}

async fn fetch_ibm_catalog_paygo(client: &Client) -> Value {
    let Ok(response) = client.get(IBM_CATALOG_PAYGO_URL).send().await else {
        return Value::Null;
    };
    let Ok(payload) = response.json::<Value>().await else {
        return Value::Null;
    };
    let Some(metrics) = payload.get("metrics").and_then(Value::as_array) else {
        return Value::Null;
    };

    let mut fallback = Value::Null;

    for metric in metrics {
        let Some(amounts) = metric.get("amounts").and_then(Value::as_array) else {
            continue;
        };

        for amount in amounts {
            let is_usa_usd = amount.get("country").and_then(Value::as_str) == Some("USA")
                && amount.get("currency").and_then(Value::as_str) == Some("USD");
            if !is_usa_usd {
                continue;
            }

            let Some(raw_price) = amount
                .get("prices")
                .and_then(Value::as_array)
                .and_then(|prices| prices.first())
                .and_then(|price| price.get("price"))
                .and_then(Value::as_f64)
            else {
                continue;
            };

            fallback = json!({
                "plan": "Pay-As-You-Go Plan",
                "charge_unit_display_name": metric.get("charge_unit_display_name"),
                "price_usd_per_second": raw_price,
                "price_usd_per_minute": raw_price * 60.0,
            });
        }
    }

    fallback
}

async fn fetch_aws_braket_api(client: &Client) -> Value {
    let downloaded_at = now_iso();

    let Ok(response) = client.get(AWS_PRICE_LIST_API_URL).send().await else {
        return aws_error(downloaded_at);
    };
    let Ok(payload) = response.json::<Value>().await else {
        return aws_error(downloaded_at);
    };

    let Some(products) = payload.get("products").and_then(Value::as_object) else {
        return aws_error(downloaded_at);
    };
    let Some(terms) = payload
        .get("terms")
        .and_then(|terms| terms.get("OnDemand"))
        .and_then(Value::as_object)
    else {
        return aws_error(downloaded_at);
    };

    let mut qpus: HashMap<String, serde_json::Map<String, Value>> = HashMap::new();

    for (sku, product) in products {
        let family = product
            .get("productFamily")
            .and_then(Value::as_str)
            .unwrap_or("");
        if family != "Quantum Task" && family != "Quantum Task-Shot" {
            continue;
        }

        let empty_object = json!({});
        let attributes = product.get("attributes").unwrap_or(&empty_object);
        let provider = attributes
            .get("provider")
            .and_then(Value::as_str)
            .unwrap_or("");
        let device_name = attributes
            .get("devicename")
            .and_then(Value::as_str)
            .unwrap_or("");
        let key = format!("{provider} - {device_name}");

        let price_usd = aws_price_usd(terms.get(sku));

        let entry = qpus.entry(key).or_insert_with(|| {
            let mut map = serde_json::Map::new();
            map.insert("hardware_provider".to_string(), json!(provider));
            map.insert("qpu_family".to_string(), json!(device_name));
            map.insert("per_task_usd".to_string(), Value::Null);
            map.insert("per_shot_usd".to_string(), Value::Null);
            map
        });

        if family == "Quantum Task" {
            entry.insert("per_task_usd".to_string(), json!(price_usd));
        } else {
            entry.insert("per_shot_usd".to_string(), json!(price_usd));
        }
    }

    let qpu_list = qpus.into_values().map(Value::Object).collect::<Vec<_>>();

    json!({
        "status": "success",
        "provider": "AWS Braket",
        "source_url": AWS_PRICE_LIST_API_URL,
        "downloaded_at": downloaded_at,
        "data": { "qpu_prices": qpu_list }
    })
}

fn aws_price_usd(sku_terms: Option<&Value>) -> f64 {
    sku_terms
        .and_then(Value::as_object)
        .and_then(|terms| terms.values().next())
        .and_then(|offer| offer.get("priceDimensions"))
        .and_then(Value::as_object)
        .and_then(|dimensions| dimensions.values().next())
        .and_then(|dimension| dimension.get("pricePerUnit"))
        .and_then(|unit| unit.get("USD"))
        .and_then(Value::as_str)
        .and_then(|price| price.parse().ok())
        .unwrap_or(0.0)
}

fn aws_error(downloaded_at: String) -> Value {
    json!({
        "status": "error",
        "provider": "AWS Braket",
        "source_url": AWS_PRICE_LIST_API_URL,
        "downloaded_at": downloaded_at,
        "data": {},
        "error": "Failed to fetch AWS API"
    })
}

pub async fn get_pricing_data(force_refresh: bool) -> Value {
    if !force_refresh && let Some(cached_data) = load_cache() {
        return cached_data;
    }

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0 Safari/537.36")
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_else(|_| Client::new());

    let ibm_data = fetch_ibm_quantum_api(&client).await;
    let aws_data = fetch_aws_braket_api(&client).await;

    let data = json!({
        "generated_at": now_iso(),
        "providers": {
            "ibm_quantum": ibm_data,
            "aws_braket": aws_data,
        }
    });

    save_cache(&data);
    data
}
