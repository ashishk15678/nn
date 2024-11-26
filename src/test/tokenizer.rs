use crate::tokenizer::tokenize;
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "Hello, world!";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].value, "Hello");
        assert_eq!(tokens[1].value, ",");
        assert_eq!(tokens[2].value, "world");
        assert_eq!(tokens[3].value, "!");
    }

    #[test]
    fn test_tokenize_with_spaces() {
        let input = "This is a test.";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].value, "This");
        assert_eq!(tokens[1].value, "is");
        assert_eq!(tokens[2].value, "a");
        assert_eq!(tokens[3].value, "test");
        assert_eq!(tokens[4].value, ".");
    }

    #[test]
    fn test_tokenize_empty_string() {
        let input = "";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_tokenize_only_punctuation() {
        let input = ".,!?";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].value, ".");
        assert_eq!(tokens[1].value, ",");
        assert_eq!(tokens[2].value, "!");
        assert_eq!(tokens[3].value, "?");
    }

    #[test]
    fn test_tokenize_mixed_characters() {
        let input = "Hello, world! 123";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens[0].value, "Hello");
        assert_eq!(tokens[1].value, ",");
        assert_eq!(tokens[2].value, "world");
        assert_eq!(tokens[3].value, "!");
        assert_eq!(tokens[4].value, "123");
    }

    #[test]
    fn test_tokenize_repeated_words() {
        let input = "test test test";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].value, "test");
        assert_eq!(tokens[0].count, 3);
    }

    #[test]
    fn test_tokenize_special_characters() {
        let input = "@#$%^&*()";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[0].value, "@");
        assert_eq!(tokens[1].value, "#");
        assert_eq!(tokens[2].value, "$");
        assert_eq!(tokens[3].value, "%");
        assert_eq!(tokens[4].value, "^");
        assert_eq!(tokens[5].value, "&");
        assert_eq!(tokens[6].value, "*");
        assert_eq!(tokens[7].value, "(");
        assert_eq!(tokens[8].value, ")");
    }
}
