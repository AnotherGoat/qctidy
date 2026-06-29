use regex::Regex;
use reqwest::Client;
use scraper::Html;
use serde_json::{Value, json};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const CACHE_FILE: &str = "pricing_cache.json";
const CACHE_EXPIRY_SECONDS: u64 = 24 * 60 * 60;
const IBM_PRICING_URL: &str = "https://www.ibm.com/quantum/pricing/";
const IBM_CATALOG_PAYGO_URL: &str =
    "https://globalcatalog.cloud.ibm.com/api/v1/5304b575-3cff-4455-90dc-ae4367762093/pricing";
const AWS_PRICE_LIST_API_URL: &str =
    "https://pricing.us-east-1.amazonaws.com/offers/v1.0/aws/AmazonBraket/current/index.json";

fn now_iso() -> String {
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).unwrap();
    format!("{}", duration.as_secs())
}

pub fn load_cache() -> Option<Value> {
    let path = Path::new(CACHE_FILE);
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
        .unwrap()
        .as_secs_f64();
    let cache_payload = json!({
        "timestamp": now,
        "downloaded_at": now_iso(),
        "expires_in_seconds": CACHE_EXPIRY_SECONDS,
        "data": data,
    });

    if let Ok(content) = serde_json::to_string_pretty(&cache_payload) {
        let _ = fs::write(CACHE_FILE, content);
    }
}

fn normalize_text(html: &str) -> String {
    let document = Html::parse_document(html);
    let text_content = document.root_element().text().collect::<Vec<_>>().join(" ");

    let re = Regex::new(r"\s+").unwrap();
    re.replace_all(&text_content, " ").to_string()
}

async fn fetch_ibm_quantum_api(client: &Client) -> Value {
    let downloaded_at = now_iso();

    // 1. Scraping HTML
    let html_result = client.get(IBM_PRICING_URL).send().await;

    let mut plans = Vec::new();

    if let Ok(resp) = html_result {
        if let Ok(html_text) = resp.text().await {
            let text = normalize_text(&html_text);

            let expected_plans = [
                "Open Plan",
                "Pay-As-You-Go Plan",
                "Flex Plan",
                "Premium Plan",
                "On-Prem Plan",
            ];

            let re_price = Regex::new(r"\$\s*([0-9]+(?:\.[0-9]+)?)\s*USD\s*/\s*minute").unwrap();
            let re_quote = Regex::new(r"(?i)requires quote|contact for quote").unwrap();

            for (i, &plan_name) in expected_plans.iter().enumerate() {
                if let Some(start) = text.find(plan_name) {
                    let mut next_positions = Vec::new();
                    for next_plan in &expected_plans[i + 1..] {
                        if let Some(p) = text[start + plan_name.len()..].find(next_plan) {
                            next_positions.push(start + plan_name.len() + p);
                        }
                    }

                    let end = next_positions
                        .into_iter()
                        .min()
                        .unwrap_or(std::cmp::min(text.len(), start + 500));
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
                    let mut price_per_min = Value::Null;
                    let mut price_per_sec = Value::Null;

                    if let Some(caps) = re_price.captures(segment) {
                        if let Ok(val) = caps[1].parse::<f64>() {
                            price_per_min = json!(val);
                            price_per_sec = json!(val / 60.0);
                            price_label = format!("${} USD / minute", val);
                        }
                    } else if re_quote.is_match(segment) {
                        price_label = "Contact for quote".to_string();
                    }

                    plans.push(json!({
                        "plan": plan_name,
                        "price_label": price_label,
                        "price_usd_per_minute": price_per_min,
                        "price_usd_per_second": price_per_sec
                    }));
                }
            }
        }
    }

    // 2. API Catalog Fallback
    let mut catalog_fallback = Value::Null;
    if let Ok(resp) = client.get(IBM_CATALOG_PAYGO_URL).send().await {
        if let Ok(payload) = resp.json::<Value>().await {
            if let Some(metrics) = payload.get("metrics").and_then(|m| m.as_array()) {
                for metric in metrics {
                    let charge_unit = metric.get("charge_unit_display_name");
                    if let Some(amounts) = metric.get("amounts").and_then(|a| a.as_array()) {
                        for amount in amounts {
                            if amount.get("country").and_then(|c| c.as_str()) == Some("USA")
                                && amount.get("currency").and_then(|c| c.as_str()) == Some("USD")
                            {
                                if let Some(prices) =
                                    amount.get("prices").and_then(|p| p.as_array())
                                {
                                    if let Some(first_price) = prices.first() {
                                        if let Some(raw_price) =
                                            first_price.get("price").and_then(|p| p.as_f64())
                                        {
                                            catalog_fallback = json!({
                                                "plan": "Pay-As-You-Go Plan",
                                                "charge_unit_display_name": charge_unit,
                                                "price_usd_per_second": raw_price,
                                                "price_usd_per_minute": raw_price * 60.0,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    json!({
        "status": "success",
        "provider": "IBM Quantum",
        "source_url": IBM_PRICING_URL,
        "downloaded_at": downloaded_at,
        "data": {
            "plans": plans,
            "catalog_paygo_fallback": catalog_fallback,
            "notes": ["Se realizó Web Scraping para todos los planes como solicitado, y fallback con API para el paygo."]
        }
    })
}

async fn fetch_aws_braket_api(client: &Client) -> Value {
    let downloaded_at = now_iso();

    if let Ok(resp) = client.get(AWS_PRICE_LIST_API_URL).send().await {
        if let Ok(payload) = resp.json::<Value>().await {
            let mut qpu_list = Vec::new();

            if let (Some(products), Some(terms)) = (
                payload.get("products").and_then(|p| p.as_object()),
                payload
                    .get("terms")
                    .and_then(|t| t.get("OnDemand"))
                    .and_then(|o| o.as_object()),
            ) {
                // Mapear family -> (provider, per_task, per_shot)
                let mut qpus: std::collections::HashMap<String, serde_json::Map<String, Value>> =
                    std::collections::HashMap::new();

                for (sku, product) in products {
                    let family = product
                        .get("productFamily")
                        .and_then(|f| f.as_str())
                        .unwrap_or("");
                    if family == "Quantum Task" || family == "Quantum Task-Shot" {
                        let empty_obj = json!({});
                        let attrs = product.get("attributes").unwrap_or(&empty_obj);
                        let provider = attrs.get("provider").and_then(|p| p.as_str()).unwrap_or("");
                        let devicename = attrs
                            .get("devicename")
                            .and_then(|d| d.as_str())
                            .unwrap_or("");

                        let key = format!("{} - {}", provider, devicename);

                        let mut price_usd = 0.0;
                        if let Some(sku_terms) = terms.get(sku).and_then(|t| t.as_object()) {
                            if let Some(offer_term) = sku_terms.values().next() {
                                if let Some(price_dims) = offer_term
                                    .get("priceDimensions")
                                    .and_then(|p| p.as_object())
                                {
                                    if let Some(price_dim) = price_dims.values().next() {
                                        if let Some(price_str) = price_dim
                                            .get("pricePerUnit")
                                            .and_then(|u| u.get("USD"))
                                            .and_then(|u| u.as_str())
                                        {
                                            price_usd = price_str.parse().unwrap_or(0.0);
                                        }
                                    }
                                }
                            }
                        }

                        let entry = qpus.entry(key.clone()).or_insert_with(|| {
                            let mut map = serde_json::Map::new();
                            map.insert("hardware_provider".to_string(), json!(provider));
                            map.insert("qpu_family".to_string(), json!(devicename));
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
                }

                for (_, v) in qpus {
                    qpu_list.push(Value::Object(v));
                }
            }

            return json!({
                "status": "success",
                "provider": "AWS Braket",
                "source_url": AWS_PRICE_LIST_API_URL,
                "downloaded_at": downloaded_at,
                "data": { "qpu_prices": qpu_list }
            });
        }
    }

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
    if !force_refresh {
        if let Some(cached_data) = load_cache() {
            return cached_data;
        }
    }

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0 Safari/537.36")
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap();

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
