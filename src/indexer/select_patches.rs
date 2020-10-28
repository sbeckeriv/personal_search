use select::document;
use std::collections::HashSet;

// from select.rs::text()
pub fn text_ignore(node: &select::node::Node, ignore_index: &HashSet<usize>) -> String {
    let mut string = String::new();
    recur(node, &mut string, ignore_index);
    return string;

    fn recur(node: &select::node::Node, string: &mut String, ignore_index: &HashSet<usize>) {
        if ignore_index.get(&node.raw().index).is_none() {
            if let Some(text) = node.as_text() {
                string.push_str(text);
            }
            for child in node.children() {
                recur(&child, string, ignore_index)
            }
        }
    }
}

// from select.rs::text()
pub fn html_ignore(node: &select::node::Node, ignore_index: &HashSet<usize>) -> String {
    let mut string = String::new();
    string.push_str(&format!("<{}>", node.name().unwrap_or("div")));
    recur(node, &mut string, ignore_index);
    string.push_str(&format!("</{}>", node.name().unwrap_or("div")));
    return string;

    fn recur(node: &select::node::Node, string: &mut String, ignore_index: &HashSet<usize>) {
        if ignore_index.get(&node.raw().index).is_none() {
            match node.raw().data {
                select::node::Data::Text(ref text) => string.push_str(text),
                select::node::Data::Element(ref _name, ref attrs) => {
                    let attrs = attrs.iter().map(|&(ref name, ref value)| (name, &**value));
                    let name = node.name().unwrap_or("div");
                    //if node name a/img keep href
                    if name == "a" {
                        string.push_str(&format!("<{} ", name,));
                        if let Some(href) = attrs
                            .clone()
                            .find(|attr| attr.0.local.to_string() == "href")
                        {
                            string.push_str(&format!(
                                "{}='{}' target='_blank'",
                                href.0.local.to_string(),
                                href.1
                            ));
                        }
                        string.push('>');
                    } else if name == "img" {
                        string.push_str(&format!("<{} ", name,));
                        if let Some(href) =
                            attrs.clone().find(|attr| attr.0.local.to_string() == "src")
                        {
                            string.push_str(&format!(
                                "{}='{}' style='max-width:100%'",
                                href.0.local.to_string(),
                                href.1
                            ));
                        }
                        string.push('>');
                    } else {
                        string.push_str(&format!("<{}>", name));
                    }

                    for child in node.children() {
                        recur(&child, string, ignore_index)
                    }

                    string.push_str(&format!("</{}>", name));
                }
                _ => {}
            }
        }
    }
}
pub fn just_content_text(document: &document::Document) -> Option<String> {
    let mut ignore = HashSet::<usize>::new();
    //remove html tags
    for name in vec![
        "script", "noscript", "style", "nav", "footer", "form", "map", "source", "canvas",
        "object", "param", "picture", "progress", "video", "svg",
    ]
    .iter()
    {
        for node in document.find(select::predicate::Name((*name).clone())) {
            ignore.insert(node.raw().index);
        }
    }

    match document.find(select::predicate::Name("body")).next() {
        Some(node) => Some(
            text_ignore(&node, &ignore)
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" "),
        ),
        _ => {
            // nothing to index
            None
        }
    }
}
// used for cleaing the view of an html string
pub fn view_body(body: &str) -> String {
    let document = document::Document::from(body);

    let mut ignore = HashSet::<usize>::new();
    //remove html tags
    for name in vec![
        "script", "noscript", "style", "nav", "footer", "form", "map", "source", "canvas",
        "object", "param", "picture", "progress", "video", "svg",
    ]
    .iter()
    {
        for node in document.find(select::predicate::Name((*name).clone())) {
            ignore.insert(node.raw().index);
        }
    }
    match document.find(select::predicate::Name("body")).next() {
        Some(node) => html_ignore(&node, &ignore),
        _ => "".to_string(),
    }
}
