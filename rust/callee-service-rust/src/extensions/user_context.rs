use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use axum::http::HeaderMap;

use super::exception_handler::ServiceError;

#[derive(Debug, Clone, PartialEq)]
pub struct UserContextRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub user_name: String,
}

impl UserContextRecord {
    pub fn to_adapter_header_map(&self) -> HashMap<String, String> {
        HashMap::from([
            ("X-TenantId".to_string(), self.tenant_id.clone()),
            ("X-OrganizationId".to_string(), self.organization_id.clone()),
            (
                "X-Auth-Request-Preferred-Username".to_string(),
                self.user_name.clone(),
            ),
        ])
    }
}

impl Default for UserContextRecord {
    fn default() -> Self {
        Self {
            tenant_id: "0".to_string(),
            organization_id: "0".to_string(),
            user_name: "anonymous".to_string(),
        }
    }
}

tokio::task_local! {
    static CONTEXT: RefCell<UserContextRecord>;
}

pub struct UserContext;

impl UserContext {
    /// Runs the future with the given context bound to it; the context is dropped once the future completes.
    pub async fn scope<F: Future>(context: UserContextRecord, f: F) -> F::Output {
        CONTEXT.scope(RefCell::new(context), f).await
    }

    pub fn create_context(headers: &HeaderMap) -> Result<UserContextRecord, ServiceError> {
        Self::create_context_from(
            header(headers, "X-TenantId"),
            header(headers, "X-OrganizationId"),
            header(headers, "X-Auth-Request-Preferred-Username"),
            header(headers, "X-UserInfo"),
        )
    }

    pub fn create_context_from(
        tenant_id: Option<&str>,
        organization_id: Option<&str>,
        user_name: Option<&str>,
        user_info: Option<&str>,
    ) -> Result<UserContextRecord, ServiceError> {
        let user_name_from_info = Self::get_user_name_from_user_info(user_info)?;
        Ok(UserContextRecord {
            tenant_id: tenant_id.unwrap_or("0").to_string(),
            organization_id: organization_id.unwrap_or("0").to_string(),
            user_name: user_name_from_info
                .or_else(|| user_name.map(str::to_string))
                .unwrap_or_else(|| "anonymous".to_string()),
        })
    }

    fn current() -> UserContextRecord {
        CONTEXT
            .try_with(|c| c.borrow().clone())
            .unwrap_or_default()
    }

    pub fn tenant_id() -> String {
        Self::current().tenant_id
    }

    pub fn set_tenant_id(tenant_id: &str) {
        let _ = CONTEXT.try_with(|c| c.borrow_mut().tenant_id = tenant_id.to_string());
    }

    pub fn organization_id() -> String {
        Self::current().organization_id
    }

    pub fn user_name() -> String {
        Self::current().user_name
    }

    pub fn adapter_header_map() -> HashMap<String, String> {
        Self::current().to_adapter_header_map()
    }

    fn get_user_name_from_user_info(user_info: Option<&str>) -> Result<Option<String>, ServiceError> {
        let Some(user_info) = user_info else {
            return Ok(None);
        };
        let decoded = URL_SAFE_NO_PAD
            .decode(user_info.trim_end_matches('='))
            .map_err(|e| ServiceError::IllegalArgument(e.to_string()))?;
        let map: HashMap<String, serde_json::Value> =
            serde_json::from_slice(&decoded).map_err(|e| ServiceError::Internal(e.to_string()))?;
        Ok(map
            .get("preferred_username")
            .and_then(|v| v.as_str())
            .map(str::to_string))
    }
}

fn header<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    headers.get(name).and_then(|v| v.to_str().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_without_headers() {
        let ctx = UserContext::create_context_from(None, None, None, None).unwrap();
        assert_eq!(ctx, UserContextRecord::default());
    }

    #[test]
    fn user_info_wins_over_user_name_header() {
        let user_info = URL_SAFE_NO_PAD.encode(r#"{"preferred_username":"walter"}"#);
        let ctx =
            UserContext::create_context_from(Some("5"), None, Some("jesse"), Some(&user_info)).unwrap();
        assert_eq!(ctx.tenant_id, "5");
        assert_eq!(ctx.user_name, "walter");
    }

    #[test]
    fn invalid_user_info_is_illegal_argument() {
        let result = UserContext::create_context_from(None, None, None, Some("!!!"));
        assert!(matches!(result, Err(ServiceError::IllegalArgument(_))));
    }

    #[tokio::test]
    async fn scope_binds_and_releases_context() {
        let ctx = UserContextRecord {
            tenant_id: "7".into(),
            organization_id: "8".into(),
            user_name: "heisenberg".into(),
        };
        UserContext::scope(ctx, async {
            assert_eq!(UserContext::tenant_id(), "7");
            UserContext::set_tenant_id("9");
            assert_eq!(UserContext::tenant_id(), "9");
            assert_eq!(UserContext::user_name(), "heisenberg");
        })
        .await;
        assert_eq!(UserContext::tenant_id(), "0");
    }
}
