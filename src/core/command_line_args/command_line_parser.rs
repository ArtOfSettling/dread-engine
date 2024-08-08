pub (in crate::core::command_line_args) fn extract_command_line_argument_u32(
    argument_name: &str,
    args: &Vec<String>
) -> Option<u32> {
    let pattern = format!("-{argument_name}=");
    args.iter()
        .find(|arg| arg.starts_with(&pattern))
        .map(|arg| arg.replace(&pattern, ""))
        .map(|arg| arg.parse::<u32>().ok())
        .flatten()
}

pub (in crate::core::command_line_args) fn extract_command_line_argument_string<'a>(
    argument_name: &str,
    args: &Vec<String>
) -> Option<String> {
    let pattern = format!("-{argument_name}=");
    return args.iter()
        .find(|arg| arg.starts_with(&pattern))
        .map(|arg| arg.replace(&pattern, ""));
}

#[cfg(test)]
mod tests {
    use crate::core::command_line_args::command_line_parser::{extract_command_line_argument_string, extract_command_line_argument_u32};

    #[test]
    fn when_valid_can_extract_and_return_u32_value() {
        let args = vec!["-width=100".to_string()];
        let result = extract_command_line_argument_u32("width", &args);
        assert_eq!(result.unwrap(), 100);
    }

    #[test]
    fn when_invalid_string_cannot_extract_u32_value_returns_none() {
        let args = vec!["-width=1y0".to_string()];
        let result = extract_command_line_argument_u32("width", &args);
        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn when_negative_value_cannot_extract_u32_value_returns_none() {
        let args = vec!["-width=-10".to_string()];
        let result = extract_command_line_argument_u32("width", &args);
        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn when_valid_can_extract_and_return_string_value() {
        let args = vec!["-title=test title".to_string()];
        let result = extract_command_line_argument_string("title", &args);
        assert_eq!(result.unwrap(), "test title".to_string());
    }
}
