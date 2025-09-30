use serde::de::DeserializeOwned;
use reqwest::Client;
use serde_json::Value;
use crate::supabasic::error::{Result, SupabasicError}; // ✅ bring in the error type

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

    pub fn insert(mut self, json: Value) -> Self {
        self.method = Method::Insert;
        self.payload = Some(json);
        self
    }

    pub fn update(mut self, json: Value) -> Self {
        self.method = Method::Update;
        self.payload = Some(json);
        self
    }

    pub fn delete(mut self) -> Self {
        self.method = Method::Delete;
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

    // ✅ new single() method
    pub async fn single(self) -> Result<Value> {
        let val: Value = self.execute().await?;
        match val.as_array().and_then(|arr| arr.first()) {
            Some(first) => Ok(first.clone()),
            None => Err(SupabasicError::Other("no row found".to_string())),

        }
    }

    pub async fn execute(self) -> Result<Value> {
        let url = format!("{}/rest/v1/{}{}", self.client.url, self.table, self.query);

        let req = match self.method {
            Method::Select => self.client.http.get(&url),
            Method::Insert => self.client.http.post(&url).json(&self.payload),
            Method::Update => self.client.http.patch(&url).json(&self.payload),
            Method::Delete => self.client.http.delete(&url),
        };

        let res = req
            .header("apikey", &self.client.api_key)
            .header("Authorization", format!("Bearer {}", &self.client.api_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .send()
            .await?;

        Ok(res.json().await?)
    }

    pub async fn execute_typed<T: DeserializeOwned>(self) -> Result<Vec<T>> {
        let url = format!("{}/rest/v1/{}{}", self.client.url, self.table, self.query);

        let req = match self.method {
            Method::Select => self.client.http.get(&url),
            Method::Insert => self.client.http.post(&url).json(&self.payload),
            Method::Update => self.client.http.patch(&url).json(&self.payload),
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
            Method::Insert => self.client.http.post(&url).json(&self.payload),
            Method::Update => self.client.http.patch(&url).json(&self.payload),
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
}
