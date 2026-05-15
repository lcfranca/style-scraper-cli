pub fn native_role_for_tag(tag: &str) -> Option<&'static str> {
    match tag {
        "button" => Some("button"),
        "a" => Some("link"),
        "input" | "textarea" => Some("textbox"),
        "select" => Some("combobox"),
        "img" => Some("img"),
        "nav" => Some("navigation"),
        "header" => Some("banner"),
        "main" => Some("main"),
        "footer" => Some("contentinfo"),
        "aside" => Some("complementary"),
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => Some("heading"),
        _ => None,
    }
}

pub fn is_landmark(role: &str) -> bool {
    matches!(
        role,
        "banner" | "navigation" | "main" | "contentinfo" | "complementary" | "region" | "search"
    )
}
