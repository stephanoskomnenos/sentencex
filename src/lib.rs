use languages::{
    Amharic, Arabic, Armenian, Bengali, Bulgarian, Burmese, Catalan, Danish, Dutch, English,
    Finnish, French, German, Greek, Gujarati, Hindi, Italian, Japanese, Kannada, Kazakh, Language,
    Malayalam, Marathi, Polish, Portuguese, Punjabi, Slovak, Spanish, Tamil,
};
mod constants;
pub mod languages;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SentenceBoundary<'a> {
    pub start_index: usize,
    pub end_index: usize,
    pub start_byte: usize,
    pub end_byte: usize,
    pub text: &'a str,
    pub boundary_symbol: Option<String>,
    pub is_paragraph_break: bool,
}

pub fn language_factory(language_code: &str) -> Box<dyn Language> {
    let mut current_code = language_code;
    let mut visited = std::collections::HashSet::new();

    loop {
        if visited.contains(current_code) {
            current_code = "en"; // Default to English if a cycle is detected
        } else {
            visited.insert(current_code);
        }

        match current_code {
            "am" => return Box::new(Amharic {}),
            "ar" => return Box::new(Arabic {}),
            "bg" => return Box::new(Bulgarian {}),
            "bn" => return Box::new(Bengali {}),
            "ca" => return Box::new(Catalan {}),
            "da" => return Box::new(Danish {}),
            "de" => return Box::new(German {}),
            "en" => return Box::new(English {}),
            "es" => return Box::new(Spanish {}),
            "el" => return Box::new(Greek {}),
            "gu" => return Box::new(Gujarati {}),
            "hi" => return Box::new(Hindi {}),
            "hy" => return Box::new(Armenian {}),
            "ja" => return Box::new(Japanese {}),
            "ml" => return Box::new(Malayalam {}),
            "mr" => return Box::new(Marathi {}),
            "sk" => return Box::new(Slovak {}),
            "my" => return Box::new(Burmese {}),
            "nl" => return Box::new(Dutch {}),
            "pt" => return Box::new(Portuguese {}),
            "it" => return Box::new(Italian {}),
            "ta" => return Box::new(Tamil {}),
            "te" => return Box::new(Tamil {}),
            "kn" => return Box::new(Kannada {}),
            "kk" => return Box::new(Kazakh {}),
            "pa" => return Box::new(Punjabi {}),
            "pl" => return Box::new(Polish {}),
            "fr" => return Box::new(French {}),
            "fi" => return Box::new(Finnish {}),
            _ => {
                if let Some(fallbacks) = languages::get_fallbacks(current_code) {
                    for next_code in fallbacks {
                        if !visited.contains(next_code) {
                            current_code = next_code;
                            break;
                        }
                    }
                } else {
                    current_code = "en"; // Default to English if no fallbacks are found
                }
            }
        }
    }
}

/// Segments a given text into sentences based on the specified language.
///
/// # Arguments
///
/// * `language_code` - A string slice that holds the language code (e.g., "en" for English, "fr" for French).
/// * `text` - A string slice that holds the text to be segmented.
///
/// # Returns
///
/// A `Vec<&str>` containing the segmented sentences.
///
/// # Example
///
/// ```
/// use sentencex::segment;
///
/// let language_code = "en";
/// let text = "Hello world. This is a test.";
/// let sentences = segment(language_code, text);
///
/// assert_eq!(sentences, vec!["Hello world. ", "This is a test."]);
/// ```
pub fn segment<'a>(language_code: &str, text: &'a str) -> Vec<&'a str> {
    let language = language_factory(language_code);
    language.segment(text)
}

/// Returns detailed sentence boundaries for a given text based on the specified language.
///
/// This function provides low-level access to sentence boundary detection, returning
/// detailed information about each boundary including start/end indices, the text content,
/// boundary symbols, and whether the boundary represents a paragraph break.
///
/// # Arguments
///
/// * `language_code` - A string slice that holds the language code (e.g., "en" for English, "fr" for French).
/// * `text` - A string slice that holds the text to be analyzed.
///
/// # Returns
///
/// A `Vec<SentenceBoundary>` containing detailed information about each sentence boundary.
/// Each `SentenceBoundary` includes:
/// - `start_index`: The character index (Unicode scalar count) where the sentence starts
/// - `end_index`: The character index (Unicode scalar count) where the sentence ends
/// - `text`: A reference to the sentence text (zero-copy)
/// - `boundary_symbol`: The punctuation mark that ended the sentence (if any)
/// - `is_paragraph_break`: Whether this boundary represents a paragraph break ("\n\n")
///
/// # Example
///
/// ```
/// use sentencex::get_sentence_boundaries;
///
/// let language_code = "en";
/// let text = "Hello world. This is a test.\n\nNew paragraph.";
/// let boundaries = get_sentence_boundaries(language_code, text);
///
/// for boundary in boundaries {
///     println!("Text: {:?}, Start: {}, End: {}",
///              boundary.text, boundary.start_index, boundary.end_index);
/// }
/// ```
pub fn get_sentence_boundaries<'a>(
    language_code: &str,
    text: &'a str,
) -> Vec<SentenceBoundary<'a>> {
    let language = language_factory(language_code);
    language.get_sentence_boundaries(text)
}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    pub fn run_language_tests_for_language(language: &str, test_file: &str) {
        let content = fs::read_to_string(test_file).expect("Failed to read test file");
        let test_cases: Vec<&str> = content.split("===\n").collect();

        for case in test_cases {
            if case.trim().starts_with('#') {
                continue; // Skip comment lines
            }
            let parts: Vec<&str> = case.split("---\n").collect();
            if parts.len() != 2 {
                continue; // Skip malformed test cases
            }

            let input = parts[0].trim();
            let expected: Vec<&str> = parts[1].lines().map(|line| line.trim()).collect();
            let result = segment(language, input);
            let trimmed_result: Vec<String> =
                result.iter().map(|item| item.trim().to_string()).collect();

            assert_eq!(trimmed_result, expected, "Failed for input: \n{}", input);
        }
    }

    #[test]
    fn test_urdu_segment() {
        run_language_tests_for_language("ur", "tests/ur.txt");
    }
    #[test]
    fn test_chinese_segment() {
        run_language_tests_for_language("zh", "tests/zh.txt");
    }

    #[test]
    fn test_get_sentence_boundaries_with_paragraph_breaks() {
        let text = "Title\n\nSentence 1.\n\nSentence 2.";
        let boundaries = get_sentence_boundaries("en", text);

        // Should have at least 2 sentences plus paragraph breaks
        assert!(boundaries.len() >= 2);

        // Verify indices are consistent
        for i in 1..boundaries.len() {
            assert!(
                boundaries[i].start_index >= boundaries[i - 1].end_index,
                "Boundary {} starts at {} but previous ends at {}",
                i,
                boundaries[i].start_index,
                boundaries[i - 1].end_index
            );
        }

        // Verify text can be reconstructed
        let reconstructed: String = boundaries.iter().map(|b| b.text).collect();
        assert_eq!(
            reconstructed, text,
            "Reconstructed text doesn't match original"
        );

        // Check that paragraph breaks are detected
        let paragraph_breaks: Vec<_> = boundaries.iter().filter(|b| b.is_paragraph_break).collect();
        assert!(
            paragraph_breaks.len() >= 2,
            "Expected at least 2 paragraph breaks, found {}",
            paragraph_breaks.len()
        );
    }

    #[test]
    fn test_get_sentence_boundaries_with_multibyte_cjk() {
        // Test with Japanese and Chinese characters (multi-byte UTF-8)
        let text = "日本語です。\n\n中文文章。";
        let boundaries = get_sentence_boundaries("en", text);

        // Should have sentences and paragraph break
        assert!(
            boundaries.len() >= 2,
            "Expected at least 2 boundaries, got {}",
            boundaries.len()
        );

        // Verify indices don't overlap and are monotonically increasing
        for i in 1..boundaries.len() {
            assert!(
                boundaries[i].start_index >= boundaries[i - 1].end_index,
                "Boundary {} starts at {} but previous ends at {}",
                i,
                boundaries[i].start_index,
                boundaries[i - 1].end_index
            );
        }

        // Verify text can be reconstructed (most important for multi-byte UTF-8)
        let reconstructed: String = boundaries.iter().map(|b| b.text).collect();
        assert_eq!(
            reconstructed, text,
            "Reconstructed text doesn't match original.\nOriginal: {:?}\nReconstructed: {:?}",
            text, reconstructed
        );
    }

    #[test]
    fn test_get_sentence_boundaries_with_emoji() {
        // Test with emoji characters (4-byte UTF-8)
        let text = "Hello world 👋.\n\nGoodbye 👋.";
        let boundaries = get_sentence_boundaries("en", text);

        // Should have sentences and paragraph break
        assert!(
            boundaries.len() >= 2,
            "Expected at least 2 boundaries, got {}",
            boundaries.len()
        );

        // Verify text can be reconstructed (critical for emoji)
        let reconstructed: String = boundaries.iter().map(|b| b.text).collect();
        assert_eq!(
            reconstructed, text,
            "Reconstructed text doesn't match original with emojis"
        );

        // Verify boundary indices are valid
        for boundary in &boundaries {
            assert!(
                boundary.start_index <= boundary.end_index,
                "Invalid boundary: start_index {} > end_index {}",
                boundary.start_index,
                boundary.end_index
            );
        }
    }

    #[test]
    fn test_get_sentence_boundaries_with_mixed_scripts() {
        // Test with mixed ASCII, Latin extended, and CJK
        let text = "English text. Café résumé.\n\n日本語のテキスト。";
        let boundaries = get_sentence_boundaries("en", text);

        // Verify text reconstruction
        let reconstructed: String = boundaries.iter().map(|b| b.text).collect();
        assert_eq!(
            reconstructed, text,
            "Mixed script text failed reconstruction"
        );

        // Verify indices are properly ordered
        for i in 1..boundaries.len() {
            assert!(
                boundaries[i].start_index >= boundaries[i - 1].end_index,
                "Boundaries not ordered correctly at index {}",
                i
            );
        }

        // Verify all boundaries have valid indices
        for (i, boundary) in boundaries.iter().enumerate() {
            assert!(
                boundary.start_index <= boundary.end_index,
                "Boundary {} has invalid indices: {} > {}",
                i,
                boundary.start_index,
                boundary.end_index
            );
        }
    }

    #[test]
    fn test_get_sentence_boundaries_character_vs_byte_offsets() {
        // This test specifically validates that character indices (start_index/end_index)
        // are correctly distinguished from byte offsets (start_byte/end_byte)
        let text = "Short. 日本語.";
        let boundaries = get_sentence_boundaries("en", text);

        // Count actual characters in original text
        let total_chars = text.chars().count();
        let total_bytes = text.len();

        // Verify that the character indices sum correctly
        // All boundaries together should cover all characters
        let mut last_char_end = 0;
        for boundary in &boundaries {
            // Each boundary should start where previous ended (no gaps)
            assert_eq!(
                boundary.start_index, last_char_end,
                "Gap in character indices: expected start {}, got {}",
                last_char_end, boundary.start_index
            );
            last_char_end = boundary.end_index;
        }
        assert_eq!(
            last_char_end, total_chars,
            "Character coverage mismatch: ended at {}, total chars = {}",
            last_char_end, total_chars
        );

        // Similarly verify byte offsets
        let mut last_byte_end = 0;
        for boundary in &boundaries {
            assert_eq!(
                boundary.start_byte, last_byte_end,
                "Gap in byte indices: expected start {}, got {}",
                last_byte_end, boundary.start_byte
            );
            last_byte_end = boundary.end_byte;
        }
        assert_eq!(
            last_byte_end, total_bytes,
            "Byte coverage mismatch: ended at {}, total bytes = {}",
            last_byte_end, total_bytes
        );
    }

    #[test]
    fn test_segment_with_multibyte_characters() {
        // Test basic segment function with multi-byte characters
        let text = "日本語です。中文文章。";
        let sentences = segment("en", text);

        // Should segment into sentences
        assert!(sentences.len() > 0, "Should find at least one sentence");

        // Verify reconstruction
        let reconstructed: String = sentences.join("");
        assert_eq!(
            reconstructed, text,
            "Segment reconstruction failed for multi-byte text"
        );
    }

    #[test]
    fn test_boundary_symbol_detection_with_trailing_space() {
        // This test reproduces the bug from issue #35:
        // boundary_symbols are not detected if space follows boundary_symbol
        let text = "Hello world. This is a test.Another test. And another test.";
        let boundaries = get_sentence_boundaries("en", text);

        // Verify we have at least 4 boundaries (4 sentences)
        assert!(
            boundaries.len() >= 4,
            "Expected at least 4 boundaries, got {}",
            boundaries.len()
        );

        // Check each boundary has the correct boundary_symbol
        let non_paragraph: Vec<_> = boundaries
            .iter()
            .filter(|b| !b.is_paragraph_break)
            .collect();

        // All 4 sentences should have period as boundary symbol
        for (i, boundary) in non_paragraph.iter().enumerate() {
            assert!(
                boundary.boundary_symbol.is_some(),
                "Boundary {} should have a boundary_symbol, got None",
                i
            );
            assert_eq!(
                boundary.boundary_symbol.as_deref(),
                Some("."),
                "Boundary {} should have period as symbol",
                i
            );
        }
    }

    #[test]
    fn test_boundary_symbol_with_multiple_trailing_spaces() {
        // Test that multiple trailing spaces don't prevent symbol detection
        let text = "Hello.  This is another.   Yet another.";
        let boundaries = get_sentence_boundaries("en", text);

        let non_paragraph = boundaries
            .iter()
            .filter(|b| !b.is_paragraph_break)
            .collect::<Vec<_>>();

        // All sentence boundaries should have period symbol
        for (i, boundary) in non_paragraph.iter().enumerate() {
            assert_eq!(
                boundary.boundary_symbol.as_deref(),
                Some("."),
                "Boundary {} should have period symbol despite trailing spaces",
                i
            );
        }
    }

    #[test]
    fn test_boundary_symbol_with_mixed_terminators() {
        // Test various sentence terminators with trailing spaces
        let text = "Hello! How are you? I'm fine. Yes.";
        let boundaries = get_sentence_boundaries("en", text);

        let non_paragraph = boundaries
            .iter()
            .filter(|b| !b.is_paragraph_break)
            .collect::<Vec<_>>();

        // Verify we detect the different terminators
        let symbols: Vec<_> = non_paragraph
            .iter()
            .filter_map(|b| b.boundary_symbol.as_deref())
            .collect();

        assert!(
            symbols.iter().any(|&s| s == "!"),
            "Should detect exclamation mark"
        );
        assert!(
            symbols.iter().any(|&s| s == "?"),
            "Should detect question mark"
        );
        assert!(symbols.iter().any(|&s| s == "."), "Should detect period");
    }

    #[test]
    fn test_boundary_symbol_with_cjk_terminator() {
        // Test with CJK full stop (。) which is a valid sentence terminator
        let text = "日本語です。中文です。";
        let boundaries = get_sentence_boundaries("en", text);

        let non_paragraph = boundaries
            .iter()
            .filter(|b| !b.is_paragraph_break)
            .collect::<Vec<_>>();

        // Should detect CJK terminators
        let with_cjk_stop = non_paragraph
            .iter()
            .filter(|b| b.boundary_symbol.as_deref() == Some("。"))
            .count();

        assert!(
            with_cjk_stop >= 1,
            "Should detect at least one CJK full stop, got {}",
            with_cjk_stop
        );
    }

    #[test]
    fn test_boundary_symbol_with_tabs_and_spaces() {
        // Test with mixed whitespace (tabs and spaces)
        let text = "First sentence.\t\n  Second sentence. Third one!";
        let boundaries = get_sentence_boundaries("en", text);

        let non_paragraph = boundaries
            .iter()
            .filter(|b| !b.is_paragraph_break)
            .collect::<Vec<_>>();

        // All should have boundary symbols
        for (i, boundary) in non_paragraph.iter().enumerate() {
            assert!(
                boundary.boundary_symbol.is_some(),
                "Boundary {} should have boundary_symbol despite mixed whitespace",
                i
            );
        }

        // Verify reconstruction still works
        let reconstructed: String = boundaries.iter().map(|b| b.text).collect();
        assert_eq!(
            reconstructed, text,
            "Text reconstruction failed with mixed whitespace"
        );
    }
}
