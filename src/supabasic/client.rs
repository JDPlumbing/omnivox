use serde::de::DeserializeOwned;
use reqwest::Client;
use serde_json::Value;
use crate::supabasic::error::{Result, SupabasicError};

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

    pub fn from(&self, table: &str) -> QueryBuilder<'_> {
        QueryBuilder {
            client: self,
            table: table.to_string(),
            query: String::new(),
            method: Method::Select,
            payload: None,
        }
    }

    pub fn new_from_env() -> Result<Self> {
        let url = std::env::var("SUPABASE_URL")
            .map_err(|_| SupabasicError::Other("SUPABASE_URL must be set".into()))?;
        let api_key = std::env::var("SUPABASE_KEY")
            .map_err(|_| SupabasicError::Other("SUPABASE_KEY must be set".into()))?;
        Ok(Supabase::new(&url, &api_key))
    }
}

enum Method {
    Select,
    Insert,
    Update,
    Delete,
}

pub struct QueryBuilder<'a> {
    client: &'a Supabase,
    table: String,
    query: String,
    method: Method,
    payload: Option<Value>,
}

impl<'a> QueryBuilder<'a> {
    pub fn select(mut self, fields: &str) -> Self {
        self.method = Method::Select;
        self.query = format!("?select={}", fields);
        self
    }

    pub fn insert<T: serde::Serialize>(mut self, item: T) -> Self {
        self.method = Method::Insert;

        // Always wrap insert payload in array
        self.payload = Some(serde_json::json!([item]));

        // Always request rows back
        if self.query.is_empty() {
            self.query = "?select=*".to_string();
        } else if !self.query.contains("select=") {
            self.query.push('&');
            self.query.push_str("select=*");
        }

        self
    }

    pub fn update(mut self, json: Value) -> Self {
        self.method = Method::Update;
        self.payload = Some(json);

        if self.query.is_empty() {
            self.query = "?select=*".to_string();
        } else if !self.query.contains("select=") {
            self.query.push('&');
            self.query.push_str("select=*");
        }

        self
    }

    pub fn delete(mut self) -> Self {
        self.method = Method::Delete;

        if self.query.is_empty() {
            self.query = "?select=*".to_string();
        } else if !self.query.contains("select=") {
            self.query.push('&');
            self.query.push_str("select=*");
        }

        self
    }

    // Filters
    pub fn eq(mut self, column: &str, value: &str) -> Self {
        let filter = format!("{}=eq.{}", column, value);
        self.add_filter(filter);
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

    fn add_filter(&mut self, filter: String) {
        if self.query.is_empty() {
            self.query = format!("?{}", filter);
        } else {
            self.query.push('&');
            self.query.push_str(&filter);
        }
    }

    /// Return exactly one row
    pub async fn single(self) -> Result<Value> {
        let val: Value = self.execute().await?;

        if let Some(arr) = val.as_array() {
            if let Some(first) = arr.first() {
                return Ok(first.clone());
            } else {
                return Err(SupabasicError::Other("no row found".to_string()));
            }
        }

        if val.is_object() {
            return Ok(val);
        }

        Err(SupabasicError::Other(format!(
            "unexpected response shape: {:?}",
            val
        )))
    }
    pub async fn single_typed<T: DeserializeOwned>(self) -> Result<T> {
        let val = self.single().await?;
        Ok(serde_json::from_value(val)?)
    }

    pub async fn execute(self) -> Result<Value> {
        let url = format!("{}/rest/v1/{}{}", self.client.url, self.table, self.query);

        let req = match self.method {
            Method::Select => self.client.http.get(&url),
            Method::Insert => {
                if let Some(ref payload) = self.payload {
                    self.client.http.post(&url).json(payload)
                } else {
                    self.client.http.post(&url)
                }
            }
            Method::Update => {
                if let Some(ref payload) = self.payload {
                    self.client.http.patch(&url).json(payload)
                } else {
                    self.client.http.patch(&url)
                }
            }
            Method::Delete => self.client.http.delete(&url),
        };

        let res = req
            .header("apikey", &self.client.api_key)
            .header("Authorization", format!("Bearer {}", &self.client.api_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .send()
            .await?;

        let text = res.text().await?;
        eprintln!("DEBUG raw response text: {}", text);

        std::fs::write("output.json", &text).expect("Unable to write output.json");

        Ok(serde_json::from_str(&text)?)
    }

    pub async fn execute_typed<T: DeserializeOwned>(self) -> Result<Vec<T>> {
        let url = format!("{}/rest/v1/{}{}", self.client.url, self.table, self.query);

        let req = match self.method {
            Method::Select => self.client.http.get(&url),
            Method::Insert => {
                if let Some(ref payload) = self.payload {
                    self.client.http.post(&url).json(payload)
                } else {
                    self.client.http.post(&url)
                }
            }
            Method::Update => {
                if let Some(ref payload) = self.payload {
                    self.client.http.patch(&url).json(payload)
                } else {
                    self.client.http.patch(&url)
                }
            }
            Method::Delete => self.client.http.delete(&url),
        };

        let res = req
            .header("apikey", &self.client.api_key)
            .header("Authorization", format!("Bearer {}", &self.client.api_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .send()
            .await?;

        Ok(res.json::<Vec<T>>().await?)
    }

    pub async fn execute_one<T: DeserializeOwned>(self) -> Result<T> {
        let url = format!("{}/rest/v1/{}{}", self.client.url, self.table, self.query);

        let req = match self.method {
            Method::Select => self.client.http.get(&url),
            Method::Insert => {
                if let Some(ref payload) = self.payload {
                    self.client.http.post(&url).json(payload)
                } else {
                    self.client.http.post(&url)
                }
            }
            Method::Update => {
                if let Some(ref payload) = self.payload {
                    self.client.http.patch(&url).json(payload)
                } else {
                    self.client.http.patch(&url)
                }
            }
            Method::Delete => self.client.http.delete(&url),
        };

        let res = req
            .header("apikey", &self.client.api_key)
            .header("Authorization", format!("Bearer {}", &self.client.api_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .send()
            .await?;

        Ok(res.json::<T>().await?)
    }

    // convenience for null checks
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

    pub fn is_(mut self, column: &str, op: &str) -> Self {
        let filter = format!("{}=is.{}", column, op);
        self.add_filter(filter);
        self
    }

    pub fn is_not_null(mut self, column: &str) -> Self {
        let filter = format!("{}=is.not.null", column);
        self.add_filter(filter);
        self
    }
    pub fn order(mut self, column: &str) -> Self {
    if self.query.is_empty() {
        self.query = format!("?order={}", column);
    } else {
        self.query.push('&');
        self.query.push_str(&format!("order={}", column));
    }
    self
    }
    

}
