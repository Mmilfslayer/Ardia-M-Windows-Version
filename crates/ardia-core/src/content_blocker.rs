use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

use crate::error::CoreResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterRule {
    pub id: u64,
    pub rule: String,           // Original filter rule text
    pub action: FilterAction,
    pub domains: Vec<String>,   // Domains this rule applies to
    pub exclude_domains: Vec<String>,
    pub resource_types: Vec<ResourceType>,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterAction {
    Block,
    Allow,
    Redirect(String),
    UpgradeScheme,
    Noop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    MainFrame,
    SubFrame,
    Stylesheet,
    Script,
    Image,
    Font,
    Object,
    XmlHttpRequest,
    Ping,
    CspReport,
    Media,
    WebSocket,
    WebTransport,
    Other,
}

impl std::str::FromStr for ResourceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "main_frame" | "document" => Ok(ResourceType::MainFrame),
            "sub_frame" => Ok(ResourceType::SubFrame),
            "stylesheet" => Ok(ResourceType::Stylesheet),
            "script" => Ok(ResourceType::Script),
            "image" => Ok(ResourceType::Image),
            "font" => Ok(ResourceType::Font),
            "object" => Ok(ResourceType::Object),
            "xmlhttprequest" | "fetch" => Ok(ResourceType::XmlHttpRequest),
            "ping" => Ok(ResourceType::Ping),
            "csp_report" => Ok(ResourceType::CspReport),
            "media" => Ok(ResourceType::Media),
            "websocket" => Ok(ResourceType::WebSocket),
            "webtransport" => Ok(ResourceType::WebTransport),
            _ => Ok(ResourceType::Other),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmeticFilter {
    pub selector: String,
    pub domains: Vec<String>,
    pub exception: bool, // Exception filter (@@)
}

pub struct ContentBlocker {
    network_filters: Vec<FilterRule>,
    cosmetic_filters: Vec<CosmeticFilter>,
    enabled: bool,
    stats: BlockerStats,
    filter_lists: Vec<FilterList>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterList {
    pub name: String,
    pub url: String,
    pub enabled: bool,
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    pub rules_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockerStats {
    pub blocked_count: u64,
    pub allowed_count: u64,
    pub redirected_count: u64,
    pub cosmetic_hidden_count: u64,
}

impl Default for BlockerStats {
    fn default() -> Self {
        Self {
            blocked_count: 0,
            allowed_count: 0,
            redirected_count: 0,
            cosmetic_hidden_count: 0,
        }
    }
}

impl ContentBlocker {
    pub async fn new() -> CoreResult<Self> {
        let mut blocker = Self {
            network_filters: Vec::new(),
            cosmetic_filters: Vec::new(),
            enabled: true,
            stats: BlockerStats::default(),
            filter_lists: Vec::new(),
        };

        // Add default filter lists
        blocker.filter_lists.push(FilterList {
            name: "EasyList".to_string(),
            url: "https://easylist.to/easylist/easylist.txt".to_string(),
            enabled: true,
            last_updated: None,
            rules_count: 0,
        });

        blocker.filter_lists.push(FilterList {
            name: "EasyPrivacy".to_string(),
            url: "https://easylist.to/easylist/easyprivacy.txt".to_string(),
            enabled: true,
            last_updated: None,
            rules_count: 0,
        });

        blocker.filter_lists.push(FilterList {
            name: "Peter Lowe's List".to_string(),
            url: "https://pgl.yoyo.org/adservers/serverlist.php?hostformat=adblockplus&showintro=0&mimetype=plaintext".to_string(),
            enabled: true,
            last_updated: None,
            rules_count: 0,
        });

        // Load built-in minimal rules
        blocker.load_builtin_rules().await?;

        Ok(blocker)
    }

    async fn load_builtin_rules(&mut self) -> CoreResult<()> {
        // Minimal built-in rules for immediate protection
        let builtin_rules = [
            // Common tracking/ad domains
            "||doubleclick.net^",
            "||googlesyndication.com^",
            "||googleadservices.com^",
            "||adsystem.amazon.com^",
            "||adserver.yahoo.com^",
            "||ads.facebook.com^",
            "||analytics.google.com^",
            "||connect.facebook.net^",
            "||pixel.facebook.com^",
            "||bat.bing.com^",
            "||scorecardresearch.com^",
            "||quantserve.com^",
            "||adnxs.com^",
            "||rubiconproject.com^",
            "||openx.net^",
            "||pubmatic.com^",
            "||criteo.com^",
            "||adroll.com^",
            "||taboola.com^",
            "||outbrain.com^",
        ];

        for (i, rule) in builtin_rules.iter().enumerate() {
            if let Ok(parsed) = Self::parse_rule(rule, i as u64) {
                self.network_filters.push(parsed);
            }
        }

        Ok(())
    }

    fn parse_rule(rule: &str, id: u64) -> CoreResult<FilterRule> {
        // Simplified ABP filter parsing
        let mut action = FilterAction::Block;
        let mut domains = Vec::new();
        let mut exclude_domains = Vec::new();
        let mut resource_types = Vec::new();
        let rule_text = rule.to_string();

        // Check for exception
        let is_exception = rule.starts_with("@@");

        // Parse domain options
        if let Some(domain_part) = rule.split('$').nth(1) {
            for option in domain_part.split(',') {
                if option.starts_with("domain=") {
                    let domain_list = &option[7..];
                    for domain in domain_list.split('|') {
                        if domain.starts_with('~') {
                            exclude_domains.push(domain[1..].to_string());
                        } else {
                            domains.push(domain.to_string());
                        }
                    }
                } else if let Ok(rt) = option.parse::<ResourceType>() {
                    resource_types.push(rt);
                }
            }
        }

        // For exception rules, action is Allow
        if is_exception {
            action = FilterAction::Allow;
        }

        Ok(FilterRule {
            id,
            rule: rule_text,
            action,
            domains,
            exclude_domains,
            resource_types,
            is_enabled: true,
        })
    }

    pub fn should_block(&self, url: &str, resource_type: ResourceType, document_url: &str) -> bool {
        if !self.enabled {
            return false;
        }

        let document_domain = Self::extract_domain(document_url);
        let url_domain = Self::extract_domain(url);

        for rule in &self.network_filters {
            if !rule.is_enabled {
                continue;
            }

            // Check resource type
            if !rule.resource_types.is_empty() && !rule.resource_types.contains(&resource_type) {
                continue;
            }

            // Check domain restrictions
            if !rule.domains.is_empty() {
                let matches = rule.domains.iter().any(|d| Self::domain_matches(&url_domain, d));
                let excluded = rule.exclude_domains.iter().any(|d| Self::domain_matches(&url_domain, d));
                if !matches || excluded {
                    continue;
                }
            }

            // Check if URL matches the rule pattern
            if Self::url_matches_pattern(url, &rule.rule) {
                match rule.action {
                    FilterAction::Block => return true,
                    FilterAction::Allow => return false, // Exception - don't block
                    _ => return true,
                }
            }
        }

        false
    }

    pub fn get_cosmetic_filters(&self, document_url: &str) -> Vec<String> {
        let document_domain = Self::extract_domain(document_url);
        let mut selectors = Vec::new();

        for filter in &self.cosmetic_filters {
            let matches = filter.domains.is_empty() ||
                filter.domains.iter().any(|d| Self::domain_matches(&document_domain, d));

            if matches && !filter.exception {
                selectors.push(filter.selector.clone());
            }
        }

        selectors
    }

    fn extract_domain(url: &str) -> String {
        if let Ok(parsed) = url::Url::parse(url) {
            if let Some(host) = parsed.host_str() {
                return host.to_string();
            }
        }
        String::new()
    }

    fn domain_matches(target: &str, pattern: &str) -> bool {
        // Simple domain matching: pattern can be exact or subdomain
        if pattern == target {
            return true;
        }
        // Check if target is a subdomain of pattern
        if target.ends_with(&format!(".{}", pattern)) {
            return true;
        }
        false
    }

    fn url_matches_pattern(url: &str, pattern: &str) -> bool {
        // Simplified pattern matching for ABP-style rules
        // Convert ABP pattern to regex
        let regex_pattern = pattern
            .replace("||", "://")
            .replace("|", "")
            .replace("^", r"(?:[^a-zA-Z0-9_.%-]|$)")
            .replace(".", r"\.")
            .replace("*", ".*");

        if let Ok(re) = regex::Regex::new(&format!("^{}$", regex_pattern)) {
            re.is_match(url)
        } else {
            false
        }
    }

    pub async fn update_filter_list(&mut self, name: &str) -> CoreResult<()> {
        // In production, fetch and parse the filter list
        // For now, just update timestamp
        if let Some(list) = self.filter_lists.iter_mut().find(|l| l.name == name) {
            list.last_updated = Some(chrono::Utc::now());
            // TODO: Download and parse actual filter list
        }
        Ok(())
    }

    pub async fn enable_filter_list(&mut self, name: &str, enabled: bool) -> CoreResult<()> {
        if let Some(list) = self.filter_lists.iter_mut().find(|l| l.name == name) {
            list.enabled = enabled;
        }
        Ok(())
    }

    pub fn get_stats(&self) -> BlockerStats {
        self.stats.clone()
    }

    pub fn increment_blocked(&mut self) {
        self.stats.blocked_count += 1;
    }

    pub fn increment_allowed(&mut self) {
        self.stats.allowed_count += 1;
    }

    pub fn increment_redirected(&mut self) {
        self.stats.redirected_count += 1;
    }

    pub fn increment_cosmetic_hidden(&mut self) {
        self.stats.cosmetic_hidden_count += 1;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn get_filter_lists(&self) -> &[FilterList] {
        &self.filter_lists
    }
}

impl Default for ContentBlocker {
    fn default() -> Self {
        Self {
            network_filters: Vec::new(),
            cosmetic_filters: Vec::new(),
            enabled: true,
            stats: BlockerStats::default(),
            filter_lists: Vec::new(),
        }
    }
}