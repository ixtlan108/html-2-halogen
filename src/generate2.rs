/*
use scraper::{Html, Selector, ElementRef, Node};
use std::fmt::Write;

fn main() {
    let html_content = r#"
        <div class="container">
            <h1 id="title">Hello World</h1>
            <p>Data content</p>
        </div>
    "#;

    let document = Html::parse_document(html_content);
    let body_selector = Selector::parse("body").unwrap();

    // In a real file, you might select the root element differently
    // For this snippet, we assume the root is the first element or iterate children
    let root = document.root_element();

    let mut output = String::new();

    // Write necessary imports
    writeln!(output, "import Halogen.HTML as HH").unwrap();
    writeln!(output, "import Halogen.HTML.Properties as HP\n").unwrap();

    // Generate the view function
    writeln!(output, "view :: HH.HTML () ()").unwrap();
    writeln!(output, "view =").unwrap();

    // Process the root element recursively
    let ps_code = element_to_halogen(root);
    writeln!(output, "  {}", ps_code).unwrap();

    println!("{}", output);
}

fn element_to_halogen(element: ElementRef) -> String {
    let tag_name = element.value().name();
    let mut attributes = Vec::new();
    let mut children = Vec::new();

    // 1. Handle Attributes
    for attr in element.value().attrs() {
        let attr_name = attr.0;
        let attr_value = attr.1;

        // Map HTML attributes to Halogen properties
        let prop = match attr_name {
            "class" => format!("HP.class_ (HP.Class \"{}\")", attr_value),
            "id" => format!("HP.id_ (HP.Id \"{}\")", attr_value),
            // Add more mappings as needed
            _ => format!("-- Unsupported attribute: {}", attr_name),
        };
        attributes.push(prop);
    }

    // 2. Handle Children (Elements and Text)
    for child in element.children() {
        match child {
            Node::Element(ref elem_ref) => {
                children.push(element_to_halogen(*elem_ref));
            },
            Node::Text(text) => {
                let content = text.trim();
                if !content.is_empty() {
                    children.push(format!("HH.text \"{}\"", content));
                }
            },
            _ => {} // Ignore comments, etc.
        }
    }

    // 3. Construct the Halogen Syntax
    let func_name = format!("HH.{}_{}", tag_name, if attributes.is_empty() && children.is_empty() { "" } else { "" });

    // Format arguments
    let attr_str = if attributes.is_empty() {
        "[]".to_string()
    } else {
        format!("[ {} ]", attributes.join(", "))
    };

    if children.is_empty() {
        // Self-closing or leaf node
        if attributes.is_empty() {
            format!("HH.{}_[]", tag_name)
        } else {
            format!("HH.{} {}", tag_name, attr_str)
        }
    } else {
        // Node with children
        // For nodes with children, Halogen usually uses the form: HH.div_ [ props ] [ children ]
        // Or HH.div_ [ children ] if no props.
        // Simplified logic for demo:

        let children_str = children.join(", ");

        if attributes.is_empty() {
            format!("HH.{}_ [ {} ]", tag_name, children_str)
        } else {
            format!("HH.{} {} [ {} ]", tag_name, attr_str, children_str)
        }
    }
}

// Add to your attribute matching logic
fn generate_event_handler(attr_name: &str, action_name: &str) -> Option<String> {
    match attr_name {
        "onclick" => Some(format!("HE.onClick (\\_ -> {})", action_name)),
        "oninput" => Some(format!("HE.onValueInput (\\str -> {} str)", action_name)),
        "onsubmit" => Some(format!("HE.onSubmit (\\ev -> {} ev)", action_name)),
        // Add more mappings as needed
        _ => None,
    }
}

// Inside your element_to_halogen function, modify the attribute loop:
for attr in element.value().attrs() {
    let attr_name = attr.0;

    if attr_name.starts_with("on") {
        // Heuristic: Convert "onclick" to "HandleClick"
        let action_name = format!("Handle{}", capitalize(&attr_name[2..]));

        if let Some(handler) = generate_event_handler(attr_name, &action_name) {
            attributes.push(handler);
        }
    } else {
        // Handle standard properties (class, id, etc.) as before
        // ...
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

*/
