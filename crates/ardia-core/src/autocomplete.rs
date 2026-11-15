use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::CoreResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocompleteResult {
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub relevance: u32,
    pub result_type: AutocompleteResultType,
    pub icon_url: Option<String>,
    pub is_bookmark: bool,
    pub is_history: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutocompleteResultType {
    Url,
    Search,
    Bookmark,
    History,
    Calculator,
    Bang,
    Extension,
}

#[derive(Debug, Clone)]
pub struct SearchEngine {
    pub name: String,
    pub keyword: String,
    pub search_url: String,
    pub suggest_url: Option<String>,
    pub icon_url: Option<String>,
    pub is_default: bool,
}

pub struct AutocompleteManager {
    search_engines: Vec<SearchEngine>,
    default_engine: String,
    custom_search_engines: HashMap<String, SearchEngine>,
}

impl AutocompleteManager {
    pub async fn new() -> CoreResult<Self> {
        let mut engines = Vec::new();

        // Default search engines
        engines.push(SearchEngine {
            name: "Google".to_string(),
            keyword: "g".to_string(),
            search_url: "https://www.google.com/search?q={}".to_string(),
            suggest_url: Some("https://suggestqueries.google.com/complete/search?client=chrome&q={}".to_string()),
            icon_url: Some("https://www.google.com/favicon.ico".to_string()),
            is_default: true,
        });

        engines.push(SearchEngine {
            name: "DuckDuckGo".to_string(),
            keyword: "d".to_string(),
            search_url: "https://duckduckgo.com/?q={}".to_string(),
            suggest_url: Some("https://ac.duckduckgo.com/ac/?q={}".to_string()),
            icon_url: Some("https://duckduckgo.com/favicon.ico".to_string()),
            is_default: false,
        });

        engines.push(SearchEngine {
            name: "Bing".to_string(),
            keyword: "b".to_string(),
            search_url: "https://www.bing.com/search?q={}".to_string(),
            suggest_url: Some("https://api.bing.com/osjson.aspx?query={}".to_string()),
            icon_url: Some("https://www.bing.com/favicon.ico".to_string()),
            is_default: false,
        });

        engines.push(SearchEngine {
            name: "GitHub".to_string(),
            keyword: "gh".to_string(),
            search_url: "https://github.com/search?q={}".to_string(),
            suggest_url: None,
            icon_url: Some("https://github.githubassets.com/favicon.ico".to_string()),
            is_default: false,
        });

        engines.push(SearchEngine {
            name: "Stack Overflow".to_string(),
            keyword: "so".to_string(),
            search_url: "https://stackoverflow.com/search?q={}".to_string(),
            suggest_url: None,
            icon_url: Some("https://cdn.sstatic.net/Sites/stackoverflow/Img/favicon.ico".to_string()),
            is_default: false,
        });

        engines.push(SearchEngine {
            name: "Wikipedia".to_string(),
            keyword: "w".to_string(),
            search_url: "https://en.wikipedia.org/w/index.php?search={}".to_string(),
            suggest_url: Some("https://en.wikipedia.org/w/api.php?action=opensearch&search={}&limit=10&format=json".to_string()),
            icon_url: Some("https://en.wikipedia.org/favicon.ico".to_string()),
            is_default: false,
        });

        Ok(Self {
            search_engines: engines,
            default_engine: "Google".to_string(),
            custom_search_engines: HashMap::new(),
        })
    }

    pub fn get_search_engines(&self) -> &[SearchEngine] {
        &self.search_engines
    }

    pub fn get_default_engine(&self) -> Option<&SearchEngine> {
        self.search_engines.iter().find(|e| e.name == self.default_engine)
            .or_else(|| self.search_engines.first())
    }

    pub fn get_engine_by_keyword(&self, keyword: &str) -> Option<&SearchEngine> {
        self.search_engines.iter().find(|e| e.keyword == keyword)
    }

    pub async fn add_custom_search_engine(&mut self, engine: SearchEngine) -> CoreResult<()> {
        if self.custom_search_engines.contains_key(&engine.keyword) {
            return Err(crate::error::CoreError::InvalidState("Keyword already exists".to_string()));
        }

        self.custom_search_engines.insert(engine.keyword.clone(), engine.clone());
        self.search_engines.push(engine);
        Ok(())
    }

    pub async fn remove_search_engine(&mut self, keyword: &str) -> CoreResult<()> {
        if keyword == "g" || keyword == "d" || keyword == "b" {
            return Err(crate::error::CoreError::InvalidState("Cannot remove default engines".to_string()));
        }

        if let Some(engine) = self.custom_search_engines.remove(keyword) {
            self.search_engines.retain(|e| e.keyword != keyword);
            if self.default_engine == engine.name {
                self.default_engine = "Google".to_string();
            }
        }
        Ok(())
    }

    pub async fn set_default_engine(&mut self, keyword: &str) -> CoreResult<()> {
        if self.search_engines.iter().any(|e| e.keyword == keyword) {
            if let Some(engine) = self.search_engines.iter().find(|e| e.keyword == keyword) {
                self.default_engine = engine.name.clone();
            }
            Ok(())
        } else {
            Err(crate::error::CoreError::NotFound("Search engine not found".to_string()))
        }
    }

    pub fn parse_input(&self, input: &str) -> AutocompleteInput {
        let input = input.trim();

        // Check for bang commands (!g, !gh, etc.)
        if input.starts_with('!') {
            let parts: Vec<&str> = input[1..].splitn(2, ' ').collect();
            let keyword = parts[0];
            let query = parts.get(1).map(|s| s.trim()).unwrap_or("");

            if let Some(engine) = self.get_engine_by_keyword(keyword) {
                return AutocompleteInput::Bang {
                    engine: engine.clone(),
                    query: query.to_string(),
                };
            }
        }

        // Check for search engine keyword (e.g., "g query")
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        if parts.len() == 2 {
            let keyword = parts[0];
            let query = parts[1];
            if let Some(engine) = self.get_engine_by_keyword(keyword) {
                return AutocompleteInput::KeywordSearch {
                    engine: engine.clone(),
                    query: query.to_string(),
                };
            }
        }

        // Check if it's a URL
        if Self::is_valid_url(input) {
            return AutocompleteInput::Url(input.to_string());
        }

        // Check if it looks like a domain
        if Self::looks_like_domain(input) {
            return AutocompleteInput::Domain(input.to_string());
        }

        // Check for calculator
        if Self::is_calculation(input) {
            return AutocompleteInput::Calculation(input.to_string());
        }

        // Default to search
        AutocompleteInput::Search(input.to_string())
    }

    fn is_valid_url(input: &str) -> bool {
        Url::parse(input).is_ok()
    }

    fn looks_like_domain(input: &str) -> bool {
        // Simple heuristic: contains a dot, no spaces, not just a number
        input.contains('.')
            && !input.contains(' ')
            && !input.parse::<f64>().is_ok()
            && input.len() > 3
    }

    fn is_calculation(input: &str) -> bool {
        // Simple heuristic: contains math operators
        input.chars().any(|c| "+-*/()%".contains(c))
            && input.chars().any(|c| c.is_ascii_digit())
    }

    pub async fn get_suggestions(
        &self,
        input: &str,
        history_manager: &crate::history::HistoryManager,
        bookmark_manager: &crate::bookmarks::BookmarkManager,
        profile_id: &str,
    ) -> CoreResult<Vec<AutocompleteResult>> {
        let parsed = self.parse_input(input);
        let mut results = Vec::new();

        match parsed {
            AutocompleteInput::Url(url) => {
                results.push(AutocompleteResult {
                    url: url.clone(),
                    title: "Go to ".to_string() + &url,
                    description: None,
                    relevance: 1000,
                    result_type: AutocompleteResultType::Url,
                    icon_url: None,
                    is_bookmark: false,
                    is_history: false,
                });
            }
            AutocompleteInput::Domain(domain) => {
                let url = if domain.starts_with("http") {
                    domain
                } else {
                    format!("https://{}", domain)
                };
                results.push(AutocompleteResult {
                    url: url.clone(),
                    title: "Go to ".to_string() + &url,
                    description: None,
                    relevance: 900,
                    result_type: AutocompleteResultType::Url,
                    icon_url: None,
                    is_bookmark: false,
                    is_history: false,
                });
            }
            AutocompleteInput::Search(query) => {
                // Get search suggestions from default engine
                if let Some(engine) = self.get_default_engine() {
                    if let Some(suggest_url) = &engine.suggest_url {
                        let url = suggest_url.replace("{}", &urlencoding::encode(&query));
                        // In a real implementation, we'd fetch suggestions from the API
                        // For now, just add the search as a result
                    }
                }

                results.push(AutocompleteResult {
                    url: self.build_search_url(&query),
                    title: format!("Search for \"{}\"", query),
                    description: Some(format!("Using {}", self.get_default_engine().map(|e| e.name.as_str()).unwrap_or("default"))),
                    relevance: 800,
                    result_type: AutocompleteResultType::Search,
                    icon_url: self.get_default_engine().and_then(|e| e.icon_url.clone()),
                    is_bookmark: false,
                    is_history: false,
                });
            }
            AutocompleteInput::KeywordSearch { engine, query } => {
                results.push(AutocompleteResult {
                    url: engine.search_url.replace("{}", &urlencoding::encode(&query)),
                    title: format!("Search {} for \"{}\"", engine.name, query),
                    description: None,
                    relevance: 900,
                    result_type: AutocompleteResultType::Search,
                    icon_url: engine.icon_url.clone(),
                    is_bookmark: false,
                    is_history: false,
                });
            }
            AutocompleteInput::Bang { engine, query } => {
                if !query.is_empty() {
                    results.push(AutocompleteResult {
                        url: engine.search_url.replace("{}", &urlencoding::encode(&query)),
                        title: format!("!{} {}", engine.keyword, query),
                        description: Some(format!("Search {} for \"{}\"", engine.name, query)),
                        relevance: 950,
                        result_type: AutocompleteResultType::Bang,
                        icon_url: engine.icon_url.clone(),
                        is_bookmark: false,
                        is_history: false,
                    });
                }
            }
            AutocompleteInput::Calculation(expr) => {
                // Simple calculator
                if let Ok(result) = Self::evaluate_expression(&expr) {
                    results.push(AutocompleteResult {
                        url: format!("calculator:{}", expr),
                        title: format!("{} = {}", expr, result),
                        description: Some("Calculator".to_string()),
                        relevance: 1000,
                        result_type: AutocompleteResultType::Calculator,
                        icon_url: None,
                        is_bookmark: false,
                        is_history: false,
                    });
                }
            }
        }

        // Add history matches
        if let Ok(history_results) = history_manager.search_history(profile_id, input, 5).await {
            for entry in history_results {
                results.push(AutocompleteResult {
                    url: entry.url.clone(),
                    title: entry.title.clone(),
                    description: Some(format!("Visited {} times", entry.visit_count)),
                    relevance: 700 - (entry.visit_count as u32).min(100),
                    result_type: AutocompleteResultType::History,
                    icon_url: Self::extract_favicon(&entry.url),
                    is_bookmark: false,
                    is_history: true,
                });
            }
        }

        // Add bookmark matches
        if let Ok(bookmark_results) = bookmark_manager.search_bookmarks(profile_id, input).await {
            for bookmark in bookmark_results {
                if let Some(url) = bookmark.url {
                    results.push(AutocompleteResult {
                        url,
                        title: bookmark.title,
                        description: Some("Bookmark".to_string()),
                        relevance: 750,
                        result_type: AutocompleteResultType::Bookmark,
                        icon_url: None,
                        is_bookmark: true,
                        is_history: false,
                    });
                }
            }
        }

        // Sort by relevance
        results.sort_by(|a, b| b.relevance.cmp(&a.relevance));

        // Limit results
        results.truncate(10);

        Ok(results)
    }

    fn build_search_url(&self, query: &str) -> String {
        if let Some(engine) = self.get_default_engine() {
            engine.search_url.replace("{}", &urlencoding::encode(query))
        } else {
            format!("https://www.google.com/search?q={}", urlencoding::encode(query))
        }
    }

    fn extract_favicon(url: &str) -> Option<String> {
        if let Ok(parsed) = Url::parse(url) {
            if let Some(host) = parsed.host_str() {
                return Some(format!("https://{}/favicon.ico", host));
            }
        }
        None
    }

    fn evaluate_expression(expr: &str) -> Result<f64, String> {
        // Very simple calculator - in production use a proper expression evaluator
        // This is just a placeholder
        let expr = expr.replace(" ", "");
        if let Ok(result) = expr.parse::<f64>() {
            return Ok(result);
        }
        Err("Not a simple number".to_string())
    }
}

#[derive(Debug, Clone)]
enum AutocompleteInput {
    Url(String),
    Domain(String),
    Search(String),
    KeywordSearch { engine: SearchEngine, query: String },
    Bang { engine: SearchEngine, query: String },
    Calculation(String),
}

impl Default for AutocompleteManager {
    fn default() -> Self {
        Self {
            search_engines: Vec::new(),
            default_engine: "Google".to_string(),
            custom_search_engines: HashMap::new(),
        }
    }
}