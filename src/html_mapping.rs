//use ego_tree::NodeRef;
use ego_tree::iter::Edge;
use scraper::node::Node;
use scraper::{ElementRef, Html, Selector};
use std::fmt;
use std::fs;

// pub struct InputData {
//     pub evt: String,
//     pub title: String,
//     pub clazz: String,
//     pub name: String,
// }

pub struct InputData {
    pub name: String,
}

pub struct ButtonData {
    pub evt: String,
    pub title: String,
    pub clazz: String,
}

pub struct DivData {
    pub clazz: String,
    pub children: Vec<Halogen>,
    pub depth: u8,
}

pub enum Halogen {
    Input(InputData),
    Button(ButtonData),
    Div(DivData),
    HalogenUndef(String),
}

impl fmt::Display for ButtonData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "button class:'{}', event:{}, title:'{}'",
            self.clazz, self.evt, self.title
        )
    }
}

impl fmt::Display for InputData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "input name:'{}'",
            self.name // "input name:'{}', class:'{}', event:{}, title:'{}'",
                      // self.name, self.clazz, self.evt, self.title
        )
    }
}

impl fmt::Display for Halogen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Halogen::Input(data) => write!(f, "{}", data),
            // Hvis det er en Button, bruk structens egen Display-implementasjon via {}
            Halogen::Button(data) => write!(f, "{}", data),

            Halogen::Div(d) => {
                writeln!(f, "div class:'{}', depth: {}", d.clazz, d.depth)?;
                // Går rekursivt igjennom alle barna og printer dem
                for child in &d.children {
                    write!(f, "\t{}", child)?; // child er Box<Halogen>, som automatisk derefereres
                }
                Ok(())
            }

            // Definer hva som skal skje hvis elementet er udefinert
            Halogen::HalogenUndef(undef) => write!(f, "Undefined Element: {}", undef),
        }
    }
}

// macro_rules! get_last_div_mut {
//     ($divs:expr, $halogens:expr) => {
//         if let Some(last_div) = $divs.last_mut() {
//             &mut last_div.children
//         } else {
//             $halogens
//         }
//     };
// }
// let x = get_last_div_mut!(divs, &mut halogens);

fn get_last_div<'a>(
    divs: &'a mut Vec<DivData>,
    halogens: &'a mut Vec<Halogen>,
) -> &'a mut Vec<Halogen> {
    if let Some(last_div) = divs.last_mut() {
        &mut last_div.children
    } else {
        halogens
    }
}

pub fn parse_html(html_file: &str, prn_enter_exit: bool) -> Vec<Halogen> {
    let mut halogens: Vec<Halogen> = Vec::new();

    let html_content = fs::read_to_string(html_file).expect("Failed to read file");

    let doc = Html::parse_document(&html_content);

    let mut divs: Vec<DivData> = Vec::new();

    //let mut cur_div: Option<DivData> = None;

    let body_selector = Selector::parse("purs-div").unwrap();

    let mut div_depth: u8 = 0;

    if let Some(body_el) = doc.root_element().select(&body_selector).next() {
        for edge in body_el.traverse() {
            match edge {
                Edge::Open(node) => {
                    if prn_enter_exit {
                        println!("Entering node: {:?}", node.value());
                    }
                    if let Some(el) = ElementRef::wrap(node) {
                        let el_tag = el.value().name();
                        let cur_halogens = get_last_div(&mut divs, &mut halogens);
                        match el_tag {
                            "purs-input" => map_input(el, cur_halogens),
                            "button" => map_button(el, cur_halogens),
                            "div" => {
                                let clazz_val = match el.attr("class") {
                                    None => "",
                                    Some(clazz) => clazz,
                                };
                                let cur_div = DivData {
                                    clazz: String::from(clazz_val),
                                    children: Vec::new(),
                                    depth: div_depth,
                                };
                                divs.push(cur_div);
                                div_depth += 1;
                            }
                            _ => halogens.push(Halogen::HalogenUndef(String::from(el_tag))),
                        };
                    }
                }
                Edge::Close(node) => {
                    if prn_enter_exit {
                        println!("Leaving node: {:?}", node.value());
                    }
                    if let Some(el) = ElementRef::wrap(node) {
                        if el.value().name() == "div" {
                            div_depth -= 1;
                            if prn_enter_exit {
                                println!("LEAVING DIV NODE");
                            }
                            if let Some(last_div) = divs.pop() {
                                if divs.is_empty() {
                                    halogens.push(Halogen::Div(last_div));
                                } else {
                                    if let Some(new_last_div) = divs.last_mut() {
                                        new_last_div.children.push(Halogen::Div(last_div));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    for hal in &halogens {
        println!("{}", hal);
    }

    halogens
}

fn map_button(el: ElementRef, result: &mut Vec<Halogen>) {
    let elx = el.value();

    let clazz_val = match elx.attr("class") {
        None => "btn ps-btn",
        Some(clazz) => clazz,
    };

    let evt_val = match elx.attr("onclick") {
        None => "Noop",
        Some(evt) => evt,
    };

    // let name_val = match elx.attr("data-name") {
    //     None => "btn",
    //     Some(name) => name,
    // };

    let title: String = el
        .children()
        .filter_map(|child| {
            if let Node::Text(text_node) = child.value() {
                Some(text_node.to_string())
            } else {
                None
            }
        })
        .collect();

    let btn = ButtonData {
        evt: String::from(evt_val),
        clazz: String::from(clazz_val),
        title: String::from(title),
    };
    result.push(Halogen::Button(btn));
}

fn map_input(el: ElementRef, result: &mut Vec<Halogen>) {
    match el.value().attr("data-name") {
        None => {}
        Some(name) => {
            let input = InputData {
                name: String::from(name),
            };
            result.push(Halogen::Input(input));
        }
    };
}

// <span class="form-group"><label class="ps-label ps-mr-1">Edition#<input type="text" class="form-control ps-input"></label></span>

/*

struct DivData {
    clazz: String,
    children: Vec<Halogen>,
}

struct ButtonData {
    label: String,
}

enum Halogen {
    Button(ButtonData),
    Div(DivData),
    HalogenUndef(String),
}

// Rekursiv funksjon som går igjennom treet og muterer elementene
fn oppdater_tre(node: &mut Halogen) {
    match node {
        // Hvis noden er en Div, muterer vi den og går dypere inn i barna
        Halogen::Div(div_data) => {
            // Endre en egenskap direkte på denne noden
            div_data.clazz = format!("{}-processed", div_data.clazz);

            // Iterer muterbart over alle barna i vektoren via .iter_mut()
            for barn in div_data.children.iter_mut() {
                oppdater_tre(barn); // Rekursivt kall for hvert barn
            }
        }
        // Hvis noden er en knapp, kan vi mutere knappedataene
        Halogen::Button(button_data) => {
            button_data.label = button_data.label.to_uppercase();
        }
        // Hvis noden er udefinert tekst, endrer vi teksten direkte
        Halogen::HalogenUndef(tekst) => {
            *tekst = String::from("Renset tekst");
        }
    }
}

fn main() {
    // 1. Bygg et lite tre (Husk 'let mut' på roten!)
    let mut rot_node = Halogen::Div(DivData {
        clazz: String::from("container"),
        children: vec![
            Halogen::Button(ButtonData {
                label: String::from("Klikk meg"),
            }),
            Halogen::Div(DivData {
                clazz: String::from("row"),
                children: vec![
                    Halogen::HalogenUndef(String::from("Gammel verdi")),
                ],
            }),
        ],
    });

    // 2. Send en muterbar referanse til den rekursive funksjonen
    oppdater_tre(&mut rot_node);
}


 */

// fn translate_node(node: NodeRef<'_, Node>, result: &mut Vec<Halogen>) {
//     if let Some(el) = ElementRef::wrap(node) {
//         // It's an HTML Element (like <div>, <a>, <p>)
//         let el_tag = el.value().name();
//         //println!("Element tag: {}", el_tag);

//         let obj = match el_tag {
//             "button" => Halogen::Button(translate_button(el)),
//             _ => Halogen::HalogenUndef,
//         };

//         result.push(obj);
//     }
// }

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
// for edge in doc.root_element().traverse() {
//     match edge {
//         Edge::Open(node) => {
//             println!("Entering node: {:?}", node.value());
//             translate_node(node, &mut output, 2);
//         }
//         Edge::Close(node) => {
//             println!("Leaving node: {:?}", node.value());
//         }
//     }
//     // if let scraper::Node::Edge::Start(node) = edge {
//     //     if let Some(el) = ElementRef::wrap(node) {
//     //         println!("Element tag: {}", el.value().name());
//     //     } else if let Node::Text(text_node) = node.value() {
//     //         // .trim() fjerner tomme linjeskift og mellomrom
//     //         if !text_node.text.trim().is_empty() {
//     //             println!("Text content: {}", text_node.text.trim());
//     //         }
//     //     }
//     // }
// }

//let selector = Selector::parse("h1").unwrap();
//for el in doc.select(&selector)

//for node in doc.root_element().children().filter_map(ElementRef::wrap) {

// for node in doc.root_element().children() {
//     translate_node(node, &mut output, 2);
// }
// else if let Node::Text(text_node) = node.value() {
//     // It's raw text
//     println!("Text content: {}", text_node.text);
// }

// match node.value() {
//     Node::Element(elem) => {}
//     Node::Text(text) => {}
//     Node::Comment(comm) => {}
//     _ => {}
// }
//let tag_name = el.value().name();
//let indent_str = "  ".repeat(indent);
// for attr in el.value().attrs() {
//     let attr_name = attr.0;
//     let attr_value = attr.1;
//     println!("Attr name: {}, attr value: {}", attr_name, attr_value);
// }
// println!(
//     "CLASS: {}, ON_CLICK: {}, TITLE: {}",
//     clazz_val, evt_val, title
// );
*/
