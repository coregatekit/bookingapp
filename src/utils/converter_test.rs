#[cfg(test)]
mod test {
    use crate::utils::converter::string_to_uuid;

    #[tokio::test]
    async fn test_convert_string_to_uuid() {
        let input = "550e8400-e29b-41d4-a716-446655440000";

        let result = string_to_uuid(input);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), input);
    }

    #[tokio::test]
    async fn test_convert_invalid_string_to_uuid() {
        let input = "invalid-uuid-string";
        
        let result = string_to_uuid(input);

        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("invalid character"));
    }
}
