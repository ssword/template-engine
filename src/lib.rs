// Each line in input can be of one of following types
#[derive(PartialEq, Debug)]
pub enum ContentType {
    Literal(String),
    TemplateVariable(ExpressionData),
    Tag(TagType),
    Unrecognized,
}

#[derive(PartialEq, Debug)]
pub enum TagType {
    ForTag,
    IfTag,
}

#[derive(PartialEq, Debug)]
pub struct ExpressionData {
    pub head: Option<String>,
    pub variable: String,
    pub tail: Option<String>,
}

pub fn get_content_type(input_line: &str) -> ContentType {
    let is_tag_expression = check_matching_pair(&input_line, "{%", "%}");
    let is_for_tag = (check_symbol_string(&input_line, "for")
        && check_symbol_string(&input_line, "in"))
        || check_symbol_string(&input_line, "endfor");
    let is_if_tag = check_symbol_string(&input_line, "if") ||
        check_symbol_string(&input_line, "endif");

    let is_template_variable = check_matching_pair(&input_line, "{{", "}}");
    let return_val;

    if is_tag_expression && is_for_tag {
        return_val = ContentType::Tag(TagType::ForTag);
    } else if is_tag_expression && is_if_tag {
        return_val = ContentType::Tag(TagType::IfTag);
    } else if is_template_variable {
        let content = get_expression_data(&input_line);
        return_val = ContentType::TemplateVariable(content);
    } else if !is_tag_expression && !is_template_variable {
        return_val = ContentType::Literal(input_line.to_string());
    } else {
        return_val = ContentType::Unrecognized;
    }
    return_val
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_literal_test() {
        let s = "<h1>Hello world</h1>";
        assert_eq!(ContentType::Literal(s.to_string()), get_content_type(s));
    }

    #[test]
    fn check_template_var_test(){
        let content = ExpressionData {
            head: Some("Hi ".to_string()),
            variable: "name".to_string(),
            tail: Some(" , welcome".to_string()),
        };

        assert_eq!(
            ContentType::TemplateVariable(content),
            get_content_type("Hi {{name}}, welcome")
        );
    }

    #[test]
    fn check_for_tag_test() {
        assert_eq!(
            ContentType::Tag(TagType::ForTag),
            get_content_type("{ % for name in names %} , welcome")
        );
    }

    #[test]
    fn check_if_tag_test() {
        assert_eq!(
            ContentType::Tag(TagType::IfTag),
            get_content_type("{% if name == 'bob' %}")
        );
    }
}