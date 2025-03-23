use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouterConfig {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entry_points: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub middlewares: Vec<String>,

    pub service: String,

    pub rule: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_syntax: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<TlsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability: Option<ObservabilityConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TlsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_resolver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<DomainConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainConfig {
    pub main: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sans: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservabilityConfig {
    pub access_logs: bool,
    pub tracing: bool,
    pub metrics: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ServiceConfig {
    Failover(FailoverConfig),
    LoadBalancer(LoadBalancerConfig),
    Mirroring(MirroringConfig),
    Weighted(WeightedConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FailoverConfig {
    pub service: String,
    pub fallback: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheckConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadBalancerConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky: Option<StickyConfig>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<ServerConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheckConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass_host_header: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_forwarding: Option<ResponseForwardingConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers_transport: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyConfig {
    pub cookie: CookieConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CookieConfig {
    pub name: String,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: String,
    pub max_age: u32,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfig {
    pub url: String,
    pub weight: u32,
    pub preserve_path: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_redirects: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseForwardingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flush_interval: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MirroringConfig {
    pub service: String,
    pub mirror_body: bool,
    pub max_body_size: u32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mirrors: Vec<MirrorConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheckConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MirrorConfig {
    pub name: String,
    pub percent: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeightedConfig {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub services: Vec<WeightedServiceConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky: Option<StickyConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheckConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeightedServiceConfig {
    pub name: String,
    pub weight: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::large_enum_variant)]
pub enum MiddlewareConfig {
    AddPrefix(AddPrefixConfig),
    BasicAuth(BasicAuthConfig),
    Buffering(BufferingConfig),
    Chain(ChainConfig),
    CircuitBreaker(CircuitBreakerConfig),
    Compress(CompressConfig),
    ContentType(ContentTypeConfig),
    DigestAuth(DigestAuthConfig),
    Errors(ErrorsConfig),
    ForwardAuth(ForwardAuthConfig),
    GrpcWeb(GrpcWebConfig),
    Headers(HeadersConfig),
    IpAllowList(IpAllowListConfig),
    IpWhiteList(IpWhiteListConfig),
    InFlightReq(InFlightReqConfig),
    PassTlsClientCert(PassTlsClientCertConfig),
    Plugin(PluginConfig),
    RateLimit(RateLimitConfig),
    RedirectRegex(RedirectRegexConfig),
    RedirectScheme(RedirectSchemeConfig),
    ReplacePath(ReplacePathConfig),
    ReplacePathRegex(ReplacePathRegexConfig),
    Retry(RetryConfig),
    StripPrefix(StripPrefixConfig),
    StripPrefixRegex(StripPrefixRegexConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddPrefixConfig {
    pub prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicAuthConfig {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_header: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_field: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BufferingConfig {
    pub max_request_body_bytes: u32,
    pub mem_request_body_bytes: u32,
    pub max_response_body_bytes: u32,
    pub mem_response_body_bytes: u32,
    pub retry_expression: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainConfig {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub middlewares: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CircuitBreakerConfig {
    pub expression: String,
    pub check_period: String,
    pub fallback_duration: String,
    pub recovery_duration: String,
    pub response_code: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompressConfig {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub excluded_content_types: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub included_content_types: Vec<String>,
    pub min_response_body_bytes: u32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub encodings: Vec<String>,
    pub default_encoding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentTypeConfig {
    pub auto_detect: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DigestAuthConfig {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_header: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_field: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorsConfig {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub status: Vec<String>,
    pub service: String,
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForwardAuthConfig {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<TlsAuthConfig>,
    pub trust_forward_header: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub auth_response_headers: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_response_headers_regex: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub auth_request_headers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub add_auth_cookies_to_response: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_field: Option<String>,
    pub forward_body: bool,
    pub max_body_size: u32,
    pub preserve_location_header: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TlsAuthConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    pub insecure_skip_verify: bool,
    pub ca_optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrpcWebConfig {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allow_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadersConfig {
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub custom_request_headers: std::collections::HashMap<String, String>,
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub custom_response_headers: std::collections::HashMap<String, String>,
    pub access_control_allow_credentials: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub access_control_allow_headers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub access_control_allow_methods: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub access_control_allow_origin_list: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub access_control_allow_origin_list_regex: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub access_control_expose_headers: Vec<String>,
    pub access_control_max_age: u32,
    pub add_vary_header: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_hosts: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub hosts_proxy_headers: Vec<String>,
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub ssl_proxy_headers: std::collections::HashMap<String, String>,
    pub sts_seconds: u32,
    pub sts_include_subdomains: bool,
    pub sts_preload: bool,
    pub force_sts_header: bool,
    pub frame_deny: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_frame_options_value: Option<String>,
    pub content_type_nosniff: bool,
    pub browser_xss_filter: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_browser_xss_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_security_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_security_policy_report_only: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_policy: Option<String>,
    pub is_development: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_policy: Option<String>,
    pub ssl_redirect: bool,
    pub ssl_temporary_redirect: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_host: Option<String>,
    pub ssl_force_host: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpAllowListConfig {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub source_range: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_strategy: Option<IpStrategyConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_status_code: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpStrategyConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub excluded_ips: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_subnet: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpWhiteListConfig {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub source_range: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_strategy: Option<IpStrategyConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InFlightReqConfig {
    pub amount: u32,
    pub source_criterion: SourceCriterionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceCriterionConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_strategy: Option<IpStrategyConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_header_name: Option<String>,
    pub request_host: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PassTlsClientCertConfig {
    pub pem: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<CertInfoConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertInfoConfig {
    pub not_after: bool,
    pub not_before: bool,
    pub sans: bool,
    pub serial_number: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<SubjectConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<IssuerConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubjectConfig {
    pub country: bool,
    pub province: bool,
    pub locality: bool,
    pub organization: bool,
    pub organizational_unit: bool,
    pub common_name: bool,
    pub serial_number: bool,
    pub domain_component: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssuerConfig {
    pub country: bool,
    pub province: bool,
    pub locality: bool,
    pub organization: bool,
    pub common_name: bool,
    pub serial_number: bool,
    pub domain_component: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginConfig {
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub plugin_conf: std::collections::HashMap<String, std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitConfig {
    pub average: u32,
    pub period: String,
    pub burst: u32,
    pub source_criterion: SourceCriterionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedirectRegexConfig {
    pub regex: String,
    pub replacement: String,
    pub permanent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedirectSchemeConfig {
    pub scheme: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    pub permanent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplacePathConfig {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplacePathRegexConfig {
    pub regex: String,
    pub replacement: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetryConfig {
    pub attempts: u32,
    pub initial_interval: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StripPrefixConfig {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub prefixes: Vec<String>,
    pub force_slash: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StripPrefixRegexConfig {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub regex: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpConfig {
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub routers: std::collections::HashMap<String, RouterConfig>,
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub services: std::collections::HashMap<String, ServiceConfig>,
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub middlewares: std::collections::HashMap<String, MiddlewareConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TraefikConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HttpConfig>,
}
