/*
use quote::quote;

fn html_to_halogen(html: &str) -> proc_macro2::TokenStream {
    // 1. Parse HTML (conceptual, using a crate like `html5ever`)
    // 2. Traverse nodes and map to Halogen DSL types
    //    - <div> -> quote! { HH.div_ [...] }
    //    - <button onClick={...}> -> quote! { HH.button [HE.onClick \_ -> Just MyAction] [...] }

    quote! {
        module Main where
        import Halogen as H
        import Halogen.HTML as HH
        import Halogen.HTML.Events as HE

        data Action = Increment | Decrement

        component :: H.Component HH.HTML Action
        component = H.mkComponent
          { initialState: \\_ -> 0
          , render: render
          , eval: H.mkEval H.defaultEval { handleAction = handleAction }
          }

        render :: Int -> H.HTML Action
        render state =
          HH.div_
            [ HH.h1_ [ HH.text ("Count: " <> show state) ]
            , HH.button [ HE.onClick \\_ -> Just Increment ] [ HH.text "Increment" ]
            , HH.button [ HE.onClick \\_ -> Just Decrement ] [ HH.text "Decrement" ]
            ]

        handleAction :: Action -> H.HalogenM Int Action ()
        handleAction Increment = H.modify_ \\state -> state + 1
        handleAction Decrement = H.modify_ \\state -> state - 1
    }
}

use scraper::{Html, Selector, ElementRef, Node, Handle};
use std::fmt::Write;

/// Main entry point
fn main() {
    let html_input = r#"
        <div class="container">
            <h1>Hello World</h1>
            <button id="btn">Click Me</button>
        </div>
    "#;

    let document = Html::parse_document(html_input);
    let mut output = String::new();

    // Start the recursive translation from the root element
    // We skip the <html> tag if present and go straight to children
    for node in document.root_element().children() {
        translate_node(node, &mut output, 0);
    }

    println!("Generated PureScript Halogen Code:\n");
    println!("{}", output);
}

/// Recursively translates an HTML node into PureScript Halogen syntax
fn translate_node(node: ElementRef, output: &mut String, indent: usize) {
    let tag_name = node.value().name();
    let indent_str = "  ".repeat(indent);

    // Map HTML tag to Halogen function name
    // Most tags are just HH.tagname_, e.g., div -> HH.div_
    let func_name = format!("HH.{}_", tag_name);

    // 1. Process Attributes
    let mut props = Vec::new();
    for attr in node.value().attrs() {
        match attr.0 {
            "class" => {
                // Convert class="foo bar" to HP.classes [ ClassName "foo", ClassName "bar" ]
                let classes: Vec<String> = attr.1.split_whitespace()
                    .map(|c| format!("ClassName \"{}\"", c))
                    .collect();
                props.push(format!("HP.classes [{}]", classes.join(", ")));
            }
            "id" => {
                props.push(format!("HP.id \"{}\"", attr.1));
            }
            // Add more attribute mappings as needed (e.g., href, src)
            _ => {
                props.push(format!("HP.attr (HA.name \"{}\") \"{}\"", attr.0, attr.1));
            }
        }
    }

    // 2. Process Event Handlers (Basic Example)
    // Note: scraper doesn't parse JS events inside 'onclick' attributes into logic.
    // We assume a generic mapping or skip them for static HTML.
    if node.value().attrs().any(|(k, _)| k.starts_with("on")) {
        props.push("HE.onClick (\\_ -> Just MyAction)".to_string());
    }

    let props_str = if !props.is_empty() {
        format!("[{}]", props.join(", "))
    } else {
        "[]".to_string()
    };

    // 3. Process Children
    let mut children_code = Vec::new();
    for child in node.children() {
        match child.value() {
            Node::Text(t) => {
                let text = t.trim();
                if !text.is_empty() {
                    children_code.push(format!("{}HH.text \"{}\"", "  ".repeat(indent + 1), escape_string(text)));
                }
            }
            Node::Element(_) => {
                // Recursive call for element nodes
                // We need to get the ElementRef again to recurse
                // Note: child is a Handle, we need to find the ElementRef in the parent context
                // or traverse differently.
                // Easier approach with scraper: use .children() which returns ElementRef directly if we filter.
            }
            _ => {}
        }
    }

    // Re-iterate specifically for Element children to recurse cleanly
    for child_elem in node.children().filter_map(|n| n.value().as_element().map(|_| n)) {
        // scraper's ElementRef doesn't allow easy recursion from Handle without re-selecting
        // A workaround is to build the string recursively by passing the ElementRef
        // But node.children() returns Handles.
        // Let's use a helper that takes Handle or stick to the initial node's children iteration

        // Correct approach: The 'node' is an ElementRef. Its .children() method returns an iterator of ElementRef.
        // Wait, node.children() in scraper returns 'Children' which yields ElementRef.
        // So the loop above `for child in node.children()` yields ElementRef.
        // My previous match on `child.value()` was slightly off because `child` IS an ElementRef.
    }

    // Correction: Re-writing the children loop for clarity and recursion
    children_code.clear();
    for child in node.children() {
        match child.value() {
            Node::Text(t) => {
                let text = t.trim();
                if !text.is_empty() {
                    children_code.push(format!("{}HH.text \"{}\"", "  ".repeat(indent + 1), escape_string(text)));
                }
            }
            Node::Element(_) => {
                let mut child_buf = String::new();
                translate_node(child, &mut child_buf, indent + 1);
                // Remove trailing newline for inline array formatting if desired,
                // but keeping structure is safer.
                children_code.push(child_buf.trim().to_string());
            }
            _ => {}
        }
    }

    // 4. Assemble the line
    // Format: HH.div_ [ props ] [ children ]
    write!(output, "{}{}", indent_str, func_name).unwrap();

    if !props.is_empty() || !children_code.is_empty() {
        write!(output, " {}", props_str).unwrap();
    } else {
        write!(output, " []").unwrap(); // Ensure empty props list if no props but maybe children? No, HH.tag_ [] []
    }

    if !children_code.is_empty() {
        write!(output, "\n{}[", indent_str).unwrap();
        for (i, child_line) in children_code.iter().enumerate() {
            if i > 0 { write!(output, ",\n").unwrap(); }
            write!(output, "{}", child_line).unwrap();
        }
        write!(output, "\n{}]{}", "  ".repeat(indent), if indent == 0 { "" } else { "" }).unwrap();
    } else {
        // If no children, Halogen often uses [] for the children list explicitly or implicitly?
        // HH.div_ props [] is explicit.
        if props.is_empty() {
             write!(output, " []").unwrap();
        }
        write!(output, " []").unwrap();
    }

    if indent == 0 {
        writeln!(output).unwrap();
    } else {
        // If we are inside a list, we don't want a newline here, the parent handles commas
        // Actually, the parent loop adds commas.
        // Let's just return the string for this node.
        // The logic above is mixing printing and buffering.
        // Refined: This function should probably return a String or push to buffer carefully.
        // For this snippet, we assume top-level or simple recursion.
        // To fix the recursion buffer issue:
        // We will just append to 'output' directly but ensure newlines are handled.
        if indent > 0 {
             // If called recursively, the caller expects a single string block.
             // The current logic appends newlines.
        }
    }
}

// Helper to escape quotes in strings
fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}


use scraper::{Html, ElementRef, Node};
use std::fmt::Write;

/// Recursively translates an HTML element and its children into PureScript Halogen code.
fn translate_node(node: ElementRef, output: &mut String, indent: usize) {
    let tag_name = node.value().name();
    let indent_str = "  ".repeat(indent);

    // 1. Map Tag
    let func_name = format!("HH.{}_", tag_name);

    // 2. Build Properties List
    let mut props = Vec::new();
    for attr in node.value().attrs() {
        match attr.0 {
            "class" => {
                let classes: Vec<String> = attr.1.split_whitespace()
                    .map(|c| format!("ClassName \"{}\"", c))
                    .collect();
                if !classes.is_empty() {
                    props.push(format!("HP.classes [{}]", classes.join(", ")));
                }
            }
            "id" => props.push(format!("HP.id \"{}\"", attr.1)),
            k if k.starts_with("on") => {
                // Map generic event to a placeholder action
                props.push("HE.onClick (\\_ -> Just MyAction)".to_string());
            }
            _ => props.push(format!("HP.attr (HA.name \"{}\") \"{}\"", attr.0, attr.1)),
        }
    }

    let props_str = if !props.is_empty() {
        format!("[{}]", props.join(", "))
    } else {
        "[]".to_string()
    };

    // 3. Process Children Recursively
    let mut children_lines = Vec::new();

    // Iterate over all child nodes (text, elements, comments)
    for child in node.children() {
        match child.value() {
            Node::Text(t) => {
                let text = t.trim();
                if !text.is_empty() {
                    children_lines.push(format!(
                        "{}HH.text \"{}\"",
                        "  ".repeat(indent + 1),
                        escape_string(text)
                    ));
                }
            }
            Node::Element(_) => {
                // 'child' in this loop is a NodeRef, but we need an ElementRef to recurse.
                // The 'children()' method on ElementRef yields ElementRef directly in newer versions,
                // or we can cast/filter.
                // In 'scraper', node.children() where node is ElementRef yields ElementRef.
                // Wait, the loop variable 'child' here is actually an ElementRef because
                // node.children() returns an iterator of ElementRef.
                // Let's verify the type: node is ElementRef. node.children() -> Children<'a, T>
                // which yields ElementRef<'a>.

                let mut child_buf = String::new();
                // Recurse
                translate_node(child, &mut child_buf, indent + 1);
                // The recursive call writes to child_buf. We need to ensure it's formatted as a list item.
                // To simplify, let's make the recursive call return a String or handle formatting here.
                // For this snippet, we assume the recursive call writes a single line or block.
                // A cleaner approach for nested lists is to have the recursive function return the string.
                children_lines.push(format!("{}{}", "  ".repeat(indent + 1), child_buf.trim()));
            }
            _ => {} // Ignore comments, doctypes, etc.
        }
    }

    // 4. Assemble Output
    write!(output, "{}{}", indent_str, func_name).unwrap();

    // Write properties
    if !props.is_empty() || !children_lines.is_empty() {
        write!(output, " {}", props_str).unwrap();
    } else {
        write!(output, " []").unwrap();
    }

    // Write children
    if !children_lines.is_empty() {
        writeln!(output).unwrap();
        writeln!(output, "{}[", indent_str).unwrap();
        for (i, line) in children_lines.iter().enumerate() {
            write!(output, "{}", line).unwrap();
            if i < children_lines.len() - 1 {
                write!(output, ",").unwrap();
            }
            writeln!(output).unwrap();
        }
        write!(output, "{}]", indent_str).unwrap();
    } else {
        // Explicit empty children list if props exist, or if strict syntax is preferred
        if props.is_empty() {
             write!(output, " []").unwrap();
        } else {
             // If props exist but no children, Halogen DSL usually expects the children list too?
             // HH.div_ [props] []
             write!(output, " []").unwrap();
        }
    }
}

fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}
*/
