use serde::{Deserialize, Serialize};
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeTitle {
    pub english: Option<String>,
    pub romaji: Option<String>,
    pub native: Option<String>,
    pub user_preferred: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectAnime {
    pub id: Option<serde_json::Value>,
    pub title: Option<AnimeTitle>,
    pub year: Option<i32>,
    pub episodes: Option<i32>,
}

impl ExpectAnime {
    pub fn from_string_title(title: String, year: Option<i32>, episodes: Option<i32>) -> Self {
        Self {
            id: None,
            title: Some(AnimeTitle {
                english: Some(title),
                romaji: None,
                native: None,
                user_preferred: None,
            }),
            year,
            episodes,
        }
    }
}

#[derive(Debug, Clone)]
pub enum MatchMethod {
    ExactYearEpisodeRaw,
    ExactYearEpisodeNormalized,
    ExactYearRaw,
    ExactYearNormalized,
    Exact,
    ExactNormalized,
    LooseYear,
    Loose,
    LastResort,
    NullMethod,
}

#[derive(Debug, Clone)]
pub struct MatchResult<T> {
    pub similarity: f64,
    pub method: MatchMethod,
    pub result: T,
    pub title: Option<String>,
    pub normalized: Option<String>,
    pub year: Option<i32>,
    pub episodes: Option<i32>,
}

/// Calculates the Jaro-Winkler distance between two strings.
/// Returns a value between 0 and 1, where 1 means the strings are identical
/// and 0 means they are completely different.
pub fn jaro_winkler_distance(s1: &str, s2: &str, p: Option<f64>) -> f64 {
    let p = p.unwrap_or(0.1);
    
    if s1 == s2 {
        return 1.0;
    }
    
    if s1.is_empty() && s2.is_empty() {
        return 1.0;
    }
    
    if s1.is_empty() || s2.is_empty() {
        return 0.0;
    }
    
    let scaling_factor = p.max(0.0).min(0.25);
    let match_distance = (s1.len().max(s2.len()) / 2).saturating_sub(1);
    
    let mut s1_matches = vec![false; s1.len()];
    let mut s2_matches = vec![false; s2.len()];
    let mut matching_chars = 0;
    
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    
    // Find matches
    for (i, &ch1) in s1_chars.iter().enumerate() {
        let start = i.saturating_sub(match_distance);
        let end = (i + match_distance + 1).min(s2_chars.len());
        
        for j in start..end {
            if !s2_matches[j] && ch1 == s2_chars[j] {
                s1_matches[i] = true;
                s2_matches[j] = true;
                matching_chars += 1;
                break;
            }
        }
    }
    
    if matching_chars == 0 {
        return 0.0;
    }
    
    // Calculate transpositions
    let mut transpositions = 0;
    let mut k = 0;
    
    for (i, &is_match) in s1_matches.iter().enumerate() {
        if is_match {
            while !s2_matches[k] {
                k += 1;
            }
            
            if s1_chars[i] != s2_chars[k] {
                transpositions += 1;
            }
            
            k += 1;
        }
    }
    
    transpositions /= 2;
    
    let jaro_similarity = (matching_chars as f64 / s1.len() as f64
        + matching_chars as f64 / s2.len() as f64
        + (matching_chars - transpositions) as f64 / matching_chars as f64)
        / 3.0;
    
    // Calculate common prefix length
    let mut common_prefix_length = 0;
    let max_prefix_length = 4.min(s1_chars.len()).min(s2_chars.len());
    
    for i in 0..max_prefix_length {
        if s1_chars[i] == s2_chars[i] {
            common_prefix_length += 1;
        } else {
            break;
        }
    }
    
    jaro_similarity + common_prefix_length as f64 * scaling_factor * (1.0 - jaro_similarity)
}

/// Cleans and normalizes a given title string for comparison.
pub fn clean_title(title: Option<&str>) -> Option<String> {
    title.map(|t| {
        t.nfkc()
            .collect::<String>()
            .chars()
            .map(|c| if c.is_alphanumeric() || c.is_whitespace() { c } else { ' ' })
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
            .chars()
            .take(100)
            .collect()
    })
}

/// Sanitizes a title string by removing unnecessary words and characters for comparison.
pub fn sanitize_title(title: Option<&str>) -> Option<String> {
    title.and_then(|t| {
        let mut sanitized = t.to_lowercase();
        
        // Replace chapters with chapter
        sanitized = sanitized.replace("chapters", "chapter");
        
        // Remove specific words related to anime seasons or parts
        let season_regex = regex::Regex::new(r"\b(season|cour|part|chapter|special)\b").unwrap();
        sanitized = season_regex.replace_all(&sanitized, "").to_string();
        
        // Remove specific words related to anime seasons or parts with numbers
        let numbered_season_regex = regex::Regex::new(r"(\d+)(?:th|rd|nd|st)?\s*(?:season|cour|part|chapter|special)\b").unwrap();
        sanitized = numbered_season_regex.replace_all(&sanitized, " $1 ").to_string();
        
        // Remove non-alphanumeric characters
        sanitized = sanitized.chars()
            .map(|c| if c.is_alphanumeric() || c.is_whitespace() { c } else { ' ' })
            .collect();
        
        // Replace specific words to ensure consistency
        sanitized = sanitized.replace("yuu", "yu");
        sanitized = sanitized.replace("ouh", "oh");
        sanitized = sanitized.replace("yaa", "ya");
        
        // Remove specific words related to anime formats or additional information
        let format_regex = regex::Regex::new(r"\b(?:uncut|uncensored|dub(?:bed)?|censored|sub(?:bed)?|the final chapters)\b|\([^)]*\)|\bBD\b|\(TV\)").unwrap();
        sanitized = format_regex.replace_all(&sanitized, "").to_string();
        
        // Normalize the string to remove accents and other diacritical marks
        sanitized = sanitized.nfd()
            .filter(|c| !unicode_categories::UnicodeCategories::is_mark(*c))
            .collect();
        
        clean_title(Some(&sanitized))
    })
}

/// Gets all available titles from an anime title object
fn get_all_titles(title: &Option<AnimeTitle>) -> Vec<String> {
    match title {
        Some(t) => {
            let mut titles = Vec::new();
            if let Some(ref pref) = t.user_preferred {
                titles.push(pref.clone());
            }
            if let Some(ref eng) = t.english {
                titles.push(eng.clone());
            }
            if let Some(ref rom) = t.romaji {
                titles.push(rom.clone());
            }
            if let Some(ref nat) = t.native {
                titles.push(nat.clone());
            }
            titles.into_iter().filter(|s| !s.is_empty()).collect()
        }
        None => Vec::new(),
    }
}

/// Finds the best matching anime from a list of results based on the search criteria
pub fn find_best_match<T: Clone>(
    search: &ExpectAnime,
    results: &[T],
    get_anime_data: impl Fn(&T) -> &ExpectAnime,
) -> Option<MatchResult<T>> {
    if results.is_empty() {
        return None;
    }
    
    let search_titles = get_all_titles(&search.title);
    if search_titles.is_empty() {
        return None;
    }
    
    let search_year = search.year;
    let search_episodes = search.episodes;
    
    // Normalize search titles
    let normalized_search_titles: Vec<String> = search_titles
        .iter()
        .filter_map(|t| sanitize_title(Some(t)))
        .collect();
    
    if normalized_search_titles.is_empty() {
        return None;
    }
    
    // 1. Exact Match with year and episodes
    if let (Some(year), Some(episodes)) = (search_year, search_episodes) {
        for candidate in results {
            let candidate_data = get_anime_data(candidate);
            let candidate_titles = get_all_titles(&candidate_data.title);
            
            if candidate_data.year == Some(year) && candidate_data.episodes == Some(episodes) {
                for search_title in &search_titles {
                    if candidate_titles.contains(search_title) {
                        return Some(MatchResult {
                            similarity: 1.0,
                            method: MatchMethod::ExactYearEpisodeRaw,
                            result: candidate.clone(),
                            title: Some(search_title.clone()),
                            normalized: None,
                            year: Some(year),
                            episodes: Some(episodes),
                        });
                    }
                }
            }
        }
    }
    
    // 2. Exact Match with normalized titles, year and episodes
    if let (Some(year), Some(episodes)) = (search_year, search_episodes) {
        for candidate in results {
            let candidate_data = get_anime_data(candidate);
            let candidate_titles = get_all_titles(&candidate_data.title);
            let normalized_candidate_titles: Vec<String> = candidate_titles
                .iter()
                .filter_map(|t| sanitize_title(Some(t)))
                .collect();
            
            if candidate_data.year == Some(year) && candidate_data.episodes == Some(episodes) {
                for normalized_search_title in &normalized_search_titles {
                    if normalized_candidate_titles.contains(normalized_search_title) {
                        return Some(MatchResult {
                            similarity: 1.0,
                            method: MatchMethod::ExactYearEpisodeNormalized,
                            result: candidate.clone(),
                            title: None,
                            normalized: Some(normalized_search_title.clone()),
                            year: Some(year),
                            episodes: Some(episodes),
                        });
                    }
                }
            }
        }
    }
    
    // 3. Exact Match with year
    if let Some(year) = search_year {
        for candidate in results {
            let candidate_data = get_anime_data(candidate);
            let candidate_titles = get_all_titles(&candidate_data.title);
            
            if candidate_data.year == Some(year) {
                for search_title in &search_titles {
                    if candidate_titles.contains(search_title) {
                        return Some(MatchResult {
                            similarity: 1.0,
                            method: MatchMethod::ExactYearRaw,
                            result: candidate.clone(),
                            title: Some(search_title.clone()),
                            normalized: None,
                            year: Some(year),
                            episodes: None,
                        });
                    }
                }
            }
        }
    }
    
    // 4. Exact Match with normalized titles and year
    if let Some(year) = search_year {
        for candidate in results {
            let candidate_data = get_anime_data(candidate);
            let candidate_titles = get_all_titles(&candidate_data.title);
            let normalized_candidate_titles: Vec<String> = candidate_titles
                .iter()
                .filter_map(|t| sanitize_title(Some(t)))
                .collect();
            
            if candidate_data.year == Some(year) {
                for normalized_search_title in &normalized_search_titles {
                    if normalized_candidate_titles.contains(normalized_search_title) {
                        return Some(MatchResult {
                            similarity: 1.0,
                            method: MatchMethod::ExactYearNormalized,
                            result: candidate.clone(),
                            title: None,
                            normalized: Some(normalized_search_title.clone()),
                            year: Some(year),
                            episodes: None,
                        });
                    }
                }
            }
        }
    }
    
    // 5. Exact title match
    for candidate in results {
        let candidate_data = get_anime_data(candidate);
        let candidate_titles = get_all_titles(&candidate_data.title);
        
        for search_title in &search_titles {
            if candidate_titles.contains(search_title) {
                return Some(MatchResult {
                    similarity: 1.0,
                    method: MatchMethod::Exact,
                    result: candidate.clone(),
                    title: Some(search_title.clone()),
                    normalized: None,
                    year: None,
                    episodes: None,
                });
            }
        }
    }
    
    // 6. Exact normalized title match
    for candidate in results {
        let candidate_data = get_anime_data(candidate);
        let candidate_titles = get_all_titles(&candidate_data.title);
        let normalized_candidate_titles: Vec<String> = candidate_titles
            .iter()
            .filter_map(|t| sanitize_title(Some(t)))
            .collect();
        
        for normalized_search_title in &normalized_search_titles {
            if normalized_candidate_titles.contains(normalized_search_title) {
                return Some(MatchResult {
                    similarity: 1.0,
                    method: MatchMethod::ExactNormalized,
                    result: candidate.clone(),
                    title: None,
                    normalized: Some(normalized_search_title.clone()),
                    year: None,
                    episodes: None,
                });
            }
        }
    }
    
    // 7. Loose match with year (similarity >= 0.8 and matching year)
    if let Some(year) = search_year {
        let mut best_loose_year_match: Option<MatchResult<T>> = None;
        
        for candidate in results {
            let candidate_data = get_anime_data(candidate);
            if candidate_data.year == Some(year) {
                let candidate_titles = get_all_titles(&candidate_data.title);
                let normalized_candidate_titles: Vec<String> = candidate_titles
                    .iter()
                    .filter_map(|t| sanitize_title(Some(t)))
                    .collect();
                
                for normalized_search_title in &normalized_search_titles {
                    for normalized_candidate_title in &normalized_candidate_titles {
                        let similarity = jaro_winkler_distance(normalized_search_title, normalized_candidate_title, None);
                        
                        if similarity >= 0.8 && (best_loose_year_match.is_none() || similarity > best_loose_year_match.as_ref().unwrap().similarity) {
                            best_loose_year_match = Some(MatchResult {
                                similarity,
                                method: MatchMethod::LooseYear,
                                result: candidate.clone(),
                                title: None,
                                normalized: Some(normalized_search_title.clone()),
                                year: Some(year),
                                episodes: None,
                            });
                        }
                    }
                }
            }
        }
        
        if let Some(match_result) = best_loose_year_match {
            return Some(match_result);
        }
    }
    
    // 8. Loose match (similarity >= 0.8)
    let mut best_loose_match: Option<MatchResult<T>> = None;
    
    for candidate in results {
        let candidate_data = get_anime_data(candidate);
        let candidate_titles = get_all_titles(&candidate_data.title);
        let normalized_candidate_titles: Vec<String> = candidate_titles
            .iter()
            .filter_map(|t| sanitize_title(Some(t)))
            .collect();
        
        for normalized_search_title in &normalized_search_titles {
            for normalized_candidate_title in &normalized_candidate_titles {
                let similarity = jaro_winkler_distance(normalized_search_title, normalized_candidate_title, None);
                
                if similarity >= 0.8 && (best_loose_match.is_none() || similarity > best_loose_match.as_ref().unwrap().similarity) {
                    best_loose_match = Some(MatchResult {
                        similarity,
                        method: MatchMethod::Loose,
                        result: candidate.clone(),
                        title: None,
                        normalized: Some(normalized_search_title.clone()),
                        year: None,
                        episodes: None,
                    });
                }
            }
        }
    }
    
    if let Some(match_result) = best_loose_match {
        return Some(match_result);
    }
    
    // 9. Last resort fuzzy match (similarity >= 0.7)
    let mut best_fuzzy_match: Option<MatchResult<T>> = None;
    
    for candidate in results {
        let candidate_data = get_anime_data(candidate);
        let candidate_titles = get_all_titles(&candidate_data.title);
        let normalized_candidate_titles: Vec<String> = candidate_titles
            .iter()
            .filter_map(|t| sanitize_title(Some(t)))
            .collect();
        
        for normalized_search_title in &normalized_search_titles {
            for normalized_candidate_title in &normalized_candidate_titles {
                let similarity = jaro_winkler_distance(normalized_search_title, normalized_candidate_title, None);
                
                if similarity >= 0.7 && (best_fuzzy_match.is_none() || similarity > best_fuzzy_match.as_ref().unwrap().similarity) {
                    best_fuzzy_match = Some(MatchResult {
                        similarity,
                        method: MatchMethod::LastResort,
                        result: candidate.clone(),
                        title: None,
                        normalized: Some(normalized_search_title.clone()),
                        year: None,
                        episodes: None,
                    });
                }
            }
        }
    }
    
    if let Some(match_result) = best_fuzzy_match {
        return Some(match_result);
    }
    
    // 10. Check if there's any match with similarity >= 0.6
    let mut best_possible_match: Option<MatchResult<T>> = None;
    let mut highest_similarity = 0.0;
    
    for candidate in results {
        let candidate_data = get_anime_data(candidate);
        let candidate_titles = get_all_titles(&candidate_data.title);
        let normalized_candidate_titles: Vec<String> = candidate_titles
            .iter()
            .filter_map(|t| sanitize_title(Some(t)))
            .collect();
        
        for normalized_search_title in &normalized_search_titles {
            for normalized_candidate_title in &normalized_candidate_titles {
                let similarity = jaro_winkler_distance(normalized_search_title, normalized_candidate_title, None);
                
                if similarity > highest_similarity {
                    highest_similarity = similarity;
                    best_possible_match = Some(MatchResult {
                        similarity,
                        method: MatchMethod::NullMethod,
                        result: candidate.clone(),
                        title: None,
                        normalized: Some(normalized_search_title.clone()),
                        year: None,
                        episodes: None,
                    });
                }
            }
        }
    }
    
    // Return None if similarity is less than 0.6
    if let Some(match_result) = best_possible_match {
        if match_result.similarity >= 0.6 {
            return Some(match_result);
        }
    }
    
    None
}
