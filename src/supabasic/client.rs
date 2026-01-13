use serde::de::DeserializeOwned;
use reqwest::Client;
use serde_json::{json, Value};
use crate::supabasic::error::{Result, SupabasicError};

#[derive(Debug, Clone)]
pub struct Supabase {
    url: String,
    api_key: String,
    http: Client,
}

impl Supabase {
    pub fn new(url: &str, api_key: &str) -> Self {
        Supabase {
            url: url.to_string(),
            api_key: api_key.to_string(),
            http: Client::new(),
        }
    }

    pub fn from(&self, table: &str) -> QueryBuilder {
        QueryBuilder::new(self.clone(), table)
    }

    pub fn new_from_env() -> Result<Self> {
        let url = std::env::var("SUPABASE_URL")
            .map_err(|_| SupabasicError::Other("SUPABASE_URL must be set".into()))?;
        let api_key = std::env::var("SUPABASE_KEY")
            .map_err(|_| SupabasicError::Other("SUPABASE_KEY must be set".into()))?;
        Ok(Supabase::new(&url, &api_key))
    }

pub async fn get_user_from_jwt(
    &self,
    token: String,
) -> Result<serde_json::Value> {
    let url = format!("{}/auth/v1/user", self.url);

    let res = self
        .http
        .get(url)
        .header("apikey", &self.api_key) // ✅ use existing field
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| SupabasicError::Other(e.to_string()))?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| SupabasicError::Other(e.to_string()))?;

    eprintln!("🔐 Supabase auth response: {}", res);

    Ok(res)
}



    pub async fn rpc(&self, function_name: &str, params: serde_json::Value) -> Result<serde_json::Value> {

        let url = format!("{}/rest/v1/rpc/{}", self.url, function_name);
        let res = self
            .http               // ✅ not client
            .post(url)
            .header("apikey", &self.api_key)
            .json(&params)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        Ok(res)
    }


}

#[derive(Debug, Clone, Copy)]
enum Method {
    Select,
    Insert,
    Update,
    Delete,
}

#[derive(Debug, Clone)]
pub struct QueryBuilder {
    client: Supabase,
    table: String,
    query: String,
    method: Method,
    payload: Option<Value>,
}

impl QueryBuilder {
    pub fn new(client: Supabase, table: &str) -> Self {
        Self {
            client,
            table: table.to_string(),
            query: String::new(),
            method: Method::Select,
            payload: None,
        }
    }

    pub fn select(mut self, fields: &str) -> Self {
        let cleaned = fields
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("");
        
        if self.query.is_empty() {
            self.query = format!("?select={}", fields);
        } else {
            self.query.push('&');
            self.query.push_str(&format!("select={}", fields));
        }
        // ✅ only set method to Select if not already something else
        if matches!(self.method, Method::Select) {
            self.method = Method::Select;
        }
        self
    }

    pub fn insert<T: serde::Serialize>(mut self, item: T) -> Self {
        self.method = Method::Insert;
        self.payload = Some(serde_json::json!([item]));
        if !self.query.contains("select=") {
            self.add_filter("select=*".into());
        }
        self
    }
    pub fn insert_raw(mut self, raw_json: serde_json::Value) -> Self {
        self.method = Method::Insert;
        // Raw path — assume it's already properly wrapped
        self.payload = Some(raw_json);
        if !self.query.contains("select=") {
            self.add_filter("select=*".into());
        }
        self
    }
    pub fn update(mut self, json: Value) -> Self {
        self.payload = Some(serde_json::json!([json]));
        self.method = Method::Update; // ✅ always force Update, no matter when called

        if !self.query.contains("select=") {
            if self.query.contains('?') {
                self.query.push('&');
            } else {
                self.query.push('?');
            }
            self.query.push_str("select=*");
        }


        eprintln!("🧱 [UPDATE] Query now: {}", self.query);
        self
    }

    pub fn delete(mut self) -> Self {
        self.method = Method::Delete;
        if !self.query.contains("select=") {
            self.add_filter("select=*".into());
        }
        self
    }

    pub fn eq(mut self, column: &str, value: &str) -> Self {
        let filter = format!("{}=eq.{}", column, value);
        self.add_filter(filter);
        // ✅ do NOT change method here — keep whatever came before (Update, Delete, etc.)
        eprintln!("🧱 EQ filter added -> {}", self.query);
        self
    }

    pub fn gt(mut self, column: &str, value: &str) -> Self {
        let filter = format!("{}=gt.{}", column, value);
        self.add_filter(filter);
        self
    }

    pub fn lt(mut self, column: &str, value: &str) -> Self {
        let filter = format!("{}=lt.{}", column, value);
        self.add_filter(filter);
        self
    }

    pub fn is_null(mut self, column: &str) -> Self {
        let filter = format!("{}=is.null", column);
        self.add_filter(filter);
        self
    }

    pub fn not_null(mut self, column: &str) -> Self {
        let filter = format!("{}=not.is.null", column);
        self.add_filter(filter);
        self
    }

    pub fn order(mut self, column: &str) -> Self {
        self.add_filter(format!("order={}", column));
        self
    }

    fn add_filter(&mut self, filter: String) {
        if self.query.is_empty() {
            self.query = format!("?{}", filter);
        } else {
            self.query.push('&');
            self.query.push_str(&filter);
        }
    }

    // ========= Execute Variants =========

    pub async fn single(self) -> Result<Value> {
        let val = self.execute().await?;
        if let Some(arr) = val.as_array() {
            if let Some(first) = arr.first() {
                return Ok(first.clone());
            }
            return Err(SupabasicError::Other("no row found".to_string()));
        }
        if val.is_object() {
            Ok(val)
        } else {
            Err(SupabasicError::Other(format!(
                "unexpected response shape: {:?}",
                val
            )))
        }
    }

    pub async fn single_typed<T: DeserializeOwned>(self) -> Result<T> {
        let val = self.single().await?;
        Ok(serde_json::from_value(val)?)
    }

    pub async fn maybe_single_typed<T: DeserializeOwned>(
        self,
    ) -> Result<Option<T>> {
        let raw = self.execute().await?;

        // Try to decode into a Vec<T> because Supabase always returns arrays
        let parsed: Vec<T> = serde_json::from_value(raw.clone())
            .map_err(|e| SupabasicError::Other(format!(
                "decode error in maybe_single_typed: {e:?}, raw={raw}"
            )))?;

        Ok(parsed.into_iter().next())
    }

    pub async fn execute(self) -> Result<Value> {
        let url = format!("{}/rest/v1/{}{}", self.client.url, self.table, self.query);

        eprintln!(
            "🧠 FINAL URL [{}]: {}",
            match self.method {
                Method::Select => "SELECT",
                Method::Insert => "INSERT",
                Method::Update => "UPDATE",
                Method::Delete => "DELETE",
            },
            url
        );

        let req = match self.method {
            Method::Select => self.client.http.get(&url),
            Method::Insert => {
                let payload = self.payload.clone().unwrap_or_else(|| json!([]));
                self.client.http.post(&url).json(&payload)
            }
            Method::Update => {
                let payload = self.payload.clone().unwrap_or_else(|| json!([]));
                self.client.http.patch(&url).json(&payload)
            }
            Method::Delete => self.client.http.delete(&url),
        };

        let res = req
            .header("apikey", &self.client.api_key)
            .header("Authorization", format!("Bearer {}", &self.client.api_key))
            .header("Content-Type", "application/json")
            .header(
                "Prefer",
                "return=representation,missing=default"
            )

            .send()
            .await?;

        let status = res.status();
        let text = res.text().await?;
        eprintln!("📡 Supabase response [{}]: {}", status, text);

        if !status.is_success() {
            return Err(SupabasicError::Other(format!(
                "Supabase returned {}: {}",
                status, text
            )));
        }

        let parsed = serde_json::from_str::<Value>(&text).unwrap_or(json!({ "raw": text }));
        Ok(parsed)
    }

    pub async fn execute_typed<T: DeserializeOwned>(self) -> Result<Vec<T>> {
        let val = self.execute().await?;
        if val.is_array() {
            Ok(serde_json::from_value(val)?)
        } else {
            Ok(vec![serde_json::from_value(val)?])
        }
    }

    pub async fn execute_one<T: DeserializeOwned>(self) -> Result<T> {
        let val = self.execute().await?;
        if let Some(arr) = val.as_array() {
            if let Some(first) = arr.first() {
                return Ok(serde_json::from_value(first.clone())?);
            }
        }
        Ok(serde_json::from_value(val)?)
    }
}


