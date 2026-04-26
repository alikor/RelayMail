use std::collections::BTreeMap;
use std::time::Duration;

use relaymail_runtime::{DeliveryPolicy, StreamPolicy, WebhookAuthConfig, WebhookConfig};

use super::flat::FlatConfig;

#[derive(Clone, Debug)]
pub(crate) struct DeliveryConfig {
    pub policy: DeliveryPolicy,
    pub default_provider_chain: Vec<String>,
    pub provider_timeout: Duration,
    pub resend_api_key: Option<String>,
    pub resend_base_url: String,
    pub postmark_server_token: Option<String>,
    pub postmark_base_url: String,
    pub postmark_message_streams: BTreeMap<String, String>,
    pub smtp2go_api_key: Option<String>,
    pub smtp2go_base_url: String,
    pub aws_ses_enabled: bool,
    pub webhook: WebhookConfig,
}

impl DeliveryConfig {
    pub(crate) fn from_flat(flat: &FlatConfig) -> Self {
        let fallback_enabled = flat.fallback_enabled.unwrap_or(true);
        let primary = flat
            .primary_provider
            .clone()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "resend".into());
        let mut default_provider_chain = vec![primary];
        if fallback_enabled {
            default_provider_chain.extend(csv(flat
                .fallback_providers
                .as_deref()
                .unwrap_or("postmark,smtp2go")));
        }
        if flat.aws_ses_enabled.unwrap_or(false)
            && !default_provider_chain
                .iter()
                .any(|name| name.eq_ignore_ascii_case("ses"))
        {
            default_provider_chain.push("ses".into());
        }
        let mut streams = BTreeMap::new();
        for name in csv(flat.streams.as_deref().unwrap_or("transactional,marketing")) {
            let lower = name.to_ascii_lowercase();
            let policy = match lower.as_str() {
                "marketing" => stream_policy(
                    &lower,
                    &flat.stream_marketing_allowed_from_domains,
                    &flat.stream_marketing_from_default,
                    &flat.stream_marketing_reply_to_default,
                    flat.stream_marketing_require_unsubscribe.unwrap_or(true),
                    flat.stream_marketing_require_consent_metadata
                        .unwrap_or(true),
                    &flat.stream_marketing_provider_chain,
                ),
                _ => stream_policy(
                    &lower,
                    &flat.stream_transactional_allowed_from_domains,
                    &flat.stream_transactional_from_default,
                    &flat.stream_transactional_reply_to_default,
                    flat.stream_transactional_require_unsubscribe
                        .unwrap_or(false),
                    flat.stream_transactional_require_consent_metadata
                        .unwrap_or(false),
                    &flat.stream_transactional_provider_chain,
                ),
            };
            streams.insert(lower, policy);
        }
        Self {
            policy: DeliveryPolicy {
                streams,
                default_stream: "transactional".into(),
                fallback_enabled,
                global_max_per_minute: flat.global_max_per_minute.unwrap_or(60),
            },
            default_provider_chain,
            provider_timeout: Duration::from_secs(flat.provider_timeout_seconds.unwrap_or(10)),
            resend_api_key: env_any(&["RESEND_API_KEY", "RELAYMAIL_RESEND_API_KEY"]),
            resend_base_url: flat
                .resend_base_url
                .clone()
                .unwrap_or_else(|| "https://api.resend.com".into()),
            postmark_server_token: env_any(&[
                "POSTMARK_SERVER_TOKEN",
                "RELAYMAIL_POSTMARK_SERVER_TOKEN",
            ]),
            postmark_base_url: flat
                .postmark_base_url
                .clone()
                .unwrap_or_else(|| "https://api.postmarkapp.com".into()),
            postmark_message_streams: postmark_streams(flat),
            smtp2go_api_key: env_any(&["SMTP2GO_API_KEY", "RELAYMAIL_SMTP2GO_API_KEY"]),
            smtp2go_base_url: flat
                .smtp2go_base_url
                .clone()
                .unwrap_or_else(|| "https://api.smtp2go.com/v3".into()),
            aws_ses_enabled: flat.aws_ses_enabled.unwrap_or(false),
            webhook: WebhookConfig {
                auth: WebhookAuthConfig {
                    resend_secret: env_any(&[
                        "RESEND_WEBHOOK_SECRET",
                        "RELAYMAIL_RESEND_WEBHOOK_SECRET",
                    ]),
                    postmark_username: env_any(&[
                        "POSTMARK_WEBHOOK_USERNAME",
                        "RELAYMAIL_POSTMARK_WEBHOOK_USERNAME",
                    ]),
                    postmark_password: env_any(&[
                        "POSTMARK_WEBHOOK_PASSWORD",
                        "RELAYMAIL_POSTMARK_WEBHOOK_PASSWORD",
                    ]),
                    smtp2go_auth_token: env_any(&[
                        "SMTP2GO_WEBHOOK_AUTH_TOKEN",
                        "RELAYMAIL_SMTP2GO_WEBHOOK_AUTH_TOKEN",
                    ]),
                },
                store_raw_payloads: flat.webhook_store_raw_payloads.unwrap_or(false),
            },
        }
    }
}

fn stream_policy(
    name: &str,
    domains: &Option<String>,
    default_from: &Option<String>,
    default_reply_to: &Option<String>,
    require_unsubscribe: bool,
    require_consent_metadata: bool,
    provider_chain: &Option<String>,
) -> StreamPolicy {
    StreamPolicy {
        name: name.into(),
        allowed_from_domains: csv(domains.as_deref().unwrap_or_default()),
        default_from: default_from.clone().filter(|s| !s.is_empty()),
        default_reply_to: default_reply_to.clone().filter(|s| !s.is_empty()),
        require_unsubscribe,
        require_consent_metadata,
        provider_chain: csv(provider_chain.as_deref().unwrap_or_default()),
    }
}

fn postmark_streams(flat: &FlatConfig) -> BTreeMap<String, String> {
    let default = flat
        .postmark_message_stream
        .clone()
        .unwrap_or_else(|| "outbound".into());
    BTreeMap::from([
        (
            "transactional".into(),
            flat.postmark_transactional_message_stream
                .clone()
                .unwrap_or_else(|| default.clone()),
        ),
        (
            "marketing".into(),
            flat.postmark_marketing_message_stream
                .clone()
                .unwrap_or(default),
        ),
    ])
}

fn csv(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_ascii_lowercase())
        .collect()
}

fn env_any(names: &[&str]) -> Option<String> {
    names
        .iter()
        .find_map(|name| std::env::var(name).ok().filter(|v| !v.is_empty()))
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    struct EnvGuard {
        previous: Vec<(&'static str, Option<String>)>,
    }

    impl EnvGuard {
        fn new(pairs: &[(&'static str, Option<&str>)]) -> Self {
            let previous = pairs
                .iter()
                .map(|(name, _)| (*name, std::env::var(name).ok()))
                .collect::<Vec<_>>();
            for (name, value) in pairs {
                match value {
                    Some(value) => std::env::set_var(name, value),
                    None => std::env::remove_var(name),
                }
            }
            Self { previous }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for (name, value) in &self.previous {
                match value {
                    Some(value) => std::env::set_var(name, value),
                    None => std::env::remove_var(name),
                }
            }
        }
    }

    fn isolated_env(pairs: &[(&'static str, Option<&str>)]) -> EnvGuard {
        EnvGuard::new(pairs)
    }

    #[test]
    fn defaults_to_resend_with_transactional_and_marketing_streams() {
        let _lock = ENV_LOCK.lock().unwrap();
        let _env = isolated_env(&[
            ("RESEND_API_KEY", None),
            ("RELAYMAIL_RESEND_API_KEY", None),
            ("POSTMARK_SERVER_TOKEN", None),
            ("RELAYMAIL_POSTMARK_SERVER_TOKEN", None),
            ("SMTP2GO_API_KEY", None),
            ("RELAYMAIL_SMTP2GO_API_KEY", None),
        ]);
        let config = DeliveryConfig::from_flat(&FlatConfig::default());

        assert_eq!(
            config.default_provider_chain,
            ["resend", "postmark", "smtp2go"]
        );
        assert_eq!(config.provider_timeout, Duration::from_secs(10));
        assert_eq!(config.resend_base_url, "https://api.resend.com");
        assert_eq!(config.postmark_base_url, "https://api.postmarkapp.com");
        assert_eq!(config.smtp2go_base_url, "https://api.smtp2go.com/v3");
        assert!(!config.aws_ses_enabled);
        assert_eq!(config.policy.default_stream, "transactional");
        assert!(config.policy.fallback_enabled);
        assert_eq!(config.policy.global_max_per_minute, 60);

        let transactional = config.policy.streams.get("transactional").unwrap();
        assert_eq!(transactional.name, "transactional");
        assert!(!transactional.require_unsubscribe);
        assert!(!transactional.require_consent_metadata);

        let marketing = config.policy.streams.get("marketing").unwrap();
        assert_eq!(marketing.name, "marketing");
        assert!(marketing.require_unsubscribe);
        assert!(marketing.require_consent_metadata);
        assert_eq!(config.postmark_message_streams["transactional"], "outbound");
        assert_eq!(config.postmark_message_streams["marketing"], "outbound");
    }

    #[test]
    fn stream_overrides_and_alias_secrets_are_loaded() {
        let _lock = ENV_LOCK.lock().unwrap();
        let _env = isolated_env(&[
            ("RESEND_API_KEY", None),
            ("RELAYMAIL_RESEND_API_KEY", Some("resend-secret")),
            ("POSTMARK_SERVER_TOKEN", None),
            ("RELAYMAIL_POSTMARK_SERVER_TOKEN", Some("postmark-secret")),
            ("SMTP2GO_API_KEY", None),
            ("RELAYMAIL_SMTP2GO_API_KEY", Some("smtp2go-secret")),
            ("RESEND_WEBHOOK_SECRET", None),
            ("RELAYMAIL_RESEND_WEBHOOK_SECRET", Some("resend-hook")),
            ("POSTMARK_WEBHOOK_USERNAME", Some("postmark-user")),
            ("RELAYMAIL_POSTMARK_WEBHOOK_USERNAME", None),
            ("POSTMARK_WEBHOOK_PASSWORD", Some("postmark-pass")),
            ("RELAYMAIL_POSTMARK_WEBHOOK_PASSWORD", None),
            ("SMTP2GO_WEBHOOK_AUTH_TOKEN", Some("smtp2go-hook")),
            ("RELAYMAIL_SMTP2GO_WEBHOOK_AUTH_TOKEN", None),
        ]);
        let flat = FlatConfig {
            primary_provider: Some("postmark".into()),
            fallback_providers: Some(" smtp2go , resend ".into()),
            aws_ses_enabled: Some(true),
            provider_timeout_seconds: Some(7),
            global_max_per_minute: Some(42),
            streams: Some("Transactional,Marketing,Alerts".into()),
            stream_transactional_allowed_from_domains: Some(
                "Mail.Example.Com, mail2.example.com".into(),
            ),
            stream_transactional_from_default: Some("Example <no-reply@mail.example.com>".into()),
            stream_transactional_reply_to_default: Some(
                "Support <support@mail.example.com>".into(),
            ),
            stream_transactional_provider_chain: Some("resend,postmark".into()),
            stream_marketing_allowed_from_domains: Some("news.example.com".into()),
            stream_marketing_from_default: Some("Example <updates@news.example.com>".into()),
            stream_marketing_provider_chain: Some("postmark,smtp2go".into()),
            stream_marketing_require_unsubscribe: Some(true),
            stream_marketing_require_consent_metadata: Some(true),
            resend_base_url: Some("https://resend.test/".into()),
            postmark_base_url: Some("https://postmark.test".into()),
            postmark_message_stream: Some("default-stream".into()),
            postmark_transactional_message_stream: Some("tx-stream".into()),
            postmark_marketing_message_stream: Some("marketing-stream".into()),
            smtp2go_base_url: Some("https://smtp2go.test/v3".into()),
            webhook_store_raw_payloads: Some(true),
            ..FlatConfig::default()
        };

        let config = DeliveryConfig::from_flat(&flat);

        assert_eq!(
            config.default_provider_chain,
            ["postmark", "smtp2go", "resend", "ses"]
        );
        assert_eq!(config.provider_timeout, Duration::from_secs(7));
        assert_eq!(config.policy.global_max_per_minute, 42);
        assert_eq!(config.resend_api_key.as_deref(), Some("resend-secret"));
        assert_eq!(
            config.postmark_server_token.as_deref(),
            Some("postmark-secret")
        );
        assert_eq!(config.smtp2go_api_key.as_deref(), Some("smtp2go-secret"));
        assert_eq!(config.resend_base_url, "https://resend.test/");
        assert_eq!(config.postmark_base_url, "https://postmark.test");
        assert_eq!(config.smtp2go_base_url, "https://smtp2go.test/v3");
        assert!(config.aws_ses_enabled);
        assert!(config.webhook.store_raw_payloads);
        assert_eq!(
            config.webhook.auth.resend_secret.as_deref(),
            Some("resend-hook")
        );
        assert_eq!(
            config.webhook.auth.postmark_username.as_deref(),
            Some("postmark-user")
        );
        assert_eq!(
            config.webhook.auth.postmark_password.as_deref(),
            Some("postmark-pass")
        );
        assert_eq!(
            config.webhook.auth.smtp2go_auth_token.as_deref(),
            Some("smtp2go-hook")
        );
        assert_eq!(
            config.postmark_message_streams["transactional"],
            "tx-stream"
        );
        assert_eq!(
            config.postmark_message_streams["marketing"],
            "marketing-stream"
        );

        let transactional = config.policy.streams.get("transactional").unwrap();
        assert_eq!(
            transactional.allowed_from_domains,
            ["mail.example.com", "mail2.example.com"]
        );
        assert_eq!(
            transactional.default_from.as_deref(),
            Some("Example <no-reply@mail.example.com>")
        );
        assert_eq!(
            transactional.default_reply_to.as_deref(),
            Some("Support <support@mail.example.com>")
        );
        assert_eq!(transactional.provider_chain, ["resend", "postmark"]);

        let marketing = config.policy.streams.get("marketing").unwrap();
        assert_eq!(marketing.allowed_from_domains, ["news.example.com"]);
        assert_eq!(
            marketing.default_from.as_deref(),
            Some("Example <updates@news.example.com>")
        );
        assert_eq!(marketing.provider_chain, ["postmark", "smtp2go"]);

        let alerts = config.policy.streams.get("alerts").unwrap();
        assert_eq!(
            alerts.allowed_from_domains,
            transactional.allowed_from_domains
        );
        assert_eq!(alerts.provider_chain, transactional.provider_chain);
    }

    #[test]
    fn fallback_can_be_disabled_without_dropping_explicit_ses() {
        let _lock = ENV_LOCK.lock().unwrap();
        let _env = isolated_env(&[]);
        let flat = FlatConfig {
            primary_provider: Some("smtp2go".into()),
            fallback_providers: Some("resend,postmark".into()),
            fallback_enabled: Some(false),
            aws_ses_enabled: Some(true),
            ..FlatConfig::default()
        };

        let config = DeliveryConfig::from_flat(&flat);

        assert_eq!(config.default_provider_chain, ["smtp2go", "ses"]);
        assert!(!config.policy.fallback_enabled);
    }
}
