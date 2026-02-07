use error::Error;
use response::{
    DeleteNamespaceResponse, DeleteVectorsResponse, QueryResponse, ResponseVector, UpsertResponse,
};
use serde_json::{from_value, Value};

pub mod error;
pub mod request;
pub mod response;

const BASE_URL: &str = "https://api.turbopuffer.com/v1";

#[derive(Clone)]
pub struct Client {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

impl Client {
    /// Create a new client with the given API key.
    ///
    /// Example:
    ///
    /// ```
    /// use turbopuffer_client::Client;
    ///
    /// let api_key = "secret";
    /// let client = Client::new(api_key);
    /// ```
    ///
    /// Panics: This method panics if a TLS backend cannot be initialized, or the
    /// resolver cannot load the system configuration.
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            base_url: BASE_URL.to_string(),
            client: reqwest::Client::new(),
        }
    }

    /// Create a new client with the given API key and custom base URL.
    pub fn with_base_url(api_key: &str, base_url: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            base_url: base_url.trim_end_matches('/').to_string(),
            client: reqwest::Client::new(),
        }
    }

    /// Scope the client to a namespace. All following operations will run on
    /// this namespace.
    ///
    /// Example:
    ///
    /// ```
    /// use turbopuffer_client::Client;
    ///
    /// let ns = Client::new("secret").namespace("test");
    /// ```
    pub fn namespace<'a>(&'a self, namespace: &'a str) -> NamespacedClient<'a> {
        NamespacedClient {
            client: self,
            namespace,
        }
    }
}

pub struct NamespacedClient<'a> {
    client: &'a Client,
    namespace: &'a str,
}

impl<'a> NamespacedClient<'a> {
    /// Upsert vectors into a namespace. This creates the namespace if it does
    /// not yet have any vectors.
    ///
    /// Accepts a raw `serde_json::Value` body for maximum flexibility.
    /// For typed requests, use `upsert_typed`.
    pub async fn upsert(&self, body: &Value) -> Result<UpsertResponse, Error> {
        let url = format!("{}/vectors/{}", &self.client.base_url, &self.namespace);
        let res = self
            .client
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.client.api_key))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await?;

        let body = res.text().await.map_err(error::request_error)?;
        let value =
            serde_json::from_str::<Value>(&body).map_err(|e| error::non_json(e, body))?;

        from_value::<UpsertResponse>(value.clone())
            .map_err(|e| error::invalid_response(e, value))
    }

    /// Upsert vectors using a typed request struct.
    pub async fn upsert_typed(
        &self,
        req: &request::UpsertRequest,
    ) -> Result<UpsertResponse, Error> {
        let value = serde_json::to_value(req)
            .map_err(|e| Error::RequestError(format!("Failed to serialize request: {e}")))?;
        self.upsert(&value).await
    }

    /// Query the namespace for matching vectors.
    ///
    /// Accepts a raw `serde_json::Value` body for maximum flexibility.
    /// For typed requests, use `query_typed`.
    pub async fn query(&self, body: &Value) -> Result<QueryResponse, Error> {
        let url = format!(
            "{}/vectors/{}/query",
            &self.client.base_url, &self.namespace
        );
        let res = self
            .client
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.client.api_key))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await?;

        let body = res.text().await.map_err(error::request_error)?;
        let value =
            serde_json::from_str::<Value>(&body).map_err(|e| error::non_json(e, body))?;

        let vectors = from_value::<Vec<ResponseVector>>(value.clone())
            .map_err(|e| error::invalid_response(e, value))?;
        Ok(QueryResponse { vectors })
    }

    /// Query the namespace using a typed request struct.
    pub async fn query_typed(
        &self,
        req: &request::QueryRequest,
    ) -> Result<QueryResponse, Error> {
        let value = serde_json::to_value(req)
            .map_err(|e| Error::RequestError(format!("Failed to serialize request: {e}")))?;
        self.query(&value).await
    }

    /// Deletes the entire namespace and all related data.
    pub async fn delete_namespace(&self) -> Result<DeleteNamespaceResponse, Error> {
        let url = format!("{}/vectors/{}", &self.client.base_url, &self.namespace);
        let res = self
            .client
            .client
            .delete(url)
            .header("Authorization", format!("Bearer {}", self.client.api_key))
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let body = res.text().await.map_err(error::request_error)?;
        let value =
            serde_json::from_str::<Value>(&body).map_err(|e| error::non_json(e, body))?;

        from_value::<DeleteNamespaceResponse>(value.clone())
            .map_err(|e| error::invalid_response(e, value))
    }

    /// Delete specific vectors by their IDs.
    ///
    /// This POSTs to the upsert endpoint with a `deletes` array,
    /// which is how Turbopuffer's API handles per-vector deletes.
    ///
    /// Example:
    ///
    /// ```ignore
    /// let ns = client.namespace("test");
    /// let res = ns.delete_vectors(&[
    ///     serde_json::json!("doc-1"),
    ///     serde_json::json!("doc-2"),
    /// ]).await.unwrap();
    /// ```
    pub async fn delete_vectors(
        &self,
        ids: &[Value],
    ) -> Result<DeleteVectorsResponse, Error> {
        let body = serde_json::json!({ "deletes": ids });
        let url = format!("{}/vectors/{}", &self.client.base_url, &self.namespace);
        let res = self
            .client
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.client.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let body = res.text().await.map_err(error::request_error)?;
        let value =
            serde_json::from_str::<Value>(&body).map_err(|e| error::non_json(e, body))?;

        from_value::<DeleteVectorsResponse>(value.clone())
            .map_err(|e| error::invalid_response(e, value))
    }

    /// Delete specific vectors using a typed request struct.
    pub async fn delete_vectors_typed(
        &self,
        req: &request::DeleteVectorsRequest,
    ) -> Result<DeleteVectorsResponse, Error> {
        self.delete_vectors(&req.deletes).await
    }

    // Keep backward compatibility with the old `delete()` method name.
    #[deprecated(note = "Use `delete_namespace()` instead for clarity")]
    pub async fn delete(&self) -> Result<DeleteNamespaceResponse, Error> {
        self.delete_namespace().await
    }
}
