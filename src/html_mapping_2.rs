use scraper::{ElementRef, Html, Node, Selector};
use std::fs;

#[derive(Debug, PartialEq)]
pub enum InputType {
    InputText,
}

fn inp_type_from_str(s: &str) -> InputType {
    match s.to_lowercase().as_str() {
        "string" => InputType::InputText,
        _ => InputType::InputText,
    }
}

#[derive(Debug)]
pub struct InputData2 {
    pub name: String,
    pub title: String,
    pub evt: String,
    pub span_class: String,
    pub label_class: String,
    pub input_class: String,
    pub input_type: InputType,
}
impl InputData2 {
    fn new(
        name: &str,
        title: &str,
        span_class: &str,
        label_class: &str,
        input_class: &str,
        input_type: &str,
        evt: &str,
    ) -> Self {
        Self {
            name: String::from(name),
            title: String::from(title),
            span_class: String::from(span_class),
            label_class: String::from(label_class),
            input_class: String::from(input_class),
            input_type: inp_type_from_str(input_type),
            evt: String::from(evt),
        }
    }
}

#[derive(Debug)]
pub struct OptionData {
    pub value: String,
    pub text: String,
    //is_first: bool,
}
impl OptionData {
    fn new(value: &str, text: &str) -> Self {
        Self {
            value: String::from(value),
            text: String::from(text),
            //is_first: is_first,
        }
    }
}

#[derive(Debug)]
pub struct SelectData2 {
    pub name: String,
    pub title: String,
    pub span_class: String,
    pub label_class: String,
    pub select_class: String,
    pub evt: String,
    pub options: Vec<OptionData>,
}
impl SelectData2 {
    fn new(
        name: &str,
        title: &str,
        span_class: &str,
        label_class: &str,
        select_class: &str,
        evt: &str,
        options: Vec<OptionData>,
    ) -> Self {
        Self {
            name: String::from(name),
            title: String::from(title),
            span_class: String::from(span_class),
            label_class: String::from(label_class),
            select_class: String::from(select_class),
            evt: String::from(evt),
            options: options,
        }
    }
}

#[derive(Debug)]
pub struct SwitchData2 {}

pub enum Halogen2 {
    Input(InputData2),
    Select(SelectData2),
    Switch(SwitchData2),
}

pub fn parse_html_file(html_file: &str) -> Vec<Halogen2> {
    let html_content = fs::read_to_string(html_file).expect("Failed to read file");
    let doc = Html::parse_document(&html_content);
    parse_html(&doc)
}

pub fn parse_html(doc: &Html) -> Vec<Halogen2> {
    let mut halogens: Vec<Halogen2> = Vec::new();
    parse_input(doc, &mut halogens);
    parse_select(doc, &mut halogens);
    halogens
}

fn parse_input(doc: &Html, result: &mut Vec<Halogen2>) {
    let input_selector = Selector::parse("purs-input").unwrap();
    for input_element in doc.select(&input_selector) {
        map_input(input_element, result);
    }
}

fn map_input(el: ElementRef, result: &mut Vec<Halogen2>) {
    // <purs-input data-name="ednum"><span class="form-group"><label class="ps-label ps-mr-1">Edition#<input type="text" class="form-control ps-input" oncange="FetchMeAPizza"></label></span></purs-input>

    let name = el.value().attr("data-name").unwrap_or("na");

    if let Some(span_el) = el
        .children()
        .filter_map(ElementRef::wrap)
        .find(|elx| elx.value().name() == "span")
    {
        let span_class = match span_el.attr("class") {
            None => "",
            Some(clazz) => clazz,
        };
        //for node in span_el.descendants() {}
        let label_selector = Selector::parse("label").unwrap();
        let input_selector = Selector::parse("input").unwrap();

        let label_el = span_el.select(&label_selector).next();
        let input_el = span_el.select(&input_selector).next();

        if let Some(label) = label_el {
            let label_class = match label.attr("class") {
                None => "",
                Some(clazz) => clazz,
            };
            let label_text: String = label.text().collect::<String>().trim().to_string();

            if let Some(input) = input_el {
                let input_type = match input.attr("type") {
                    None => "text",
                    Some(inp_type) => inp_type,
                };
                let input_class = match input.attr("class") {
                    None => "",
                    Some(clazz) => clazz,
                };
                let evt_val = match input.attr("onchange") {
                    None => "",
                    Some(evt) => evt,
                };
                let inp_obj = InputData2::new(
                    name,
                    &label_text,
                    span_class,
                    label_class,
                    input_class,
                    input_type,
                    evt_val,
                );
                result.push(Halogen2::Input(inp_obj));
            }
        }
    }
}

fn parse_select(doc: &Html, result: &mut Vec<Halogen2>) {
    let select_selector = Selector::parse("purs-select").unwrap();
    for select_element in doc.select(&select_selector) {
        map_select(select_element, result);
    }
}

// fn get_direct_text(element: ElementRef) -> String {
//     element
//         .children()
//         .filter_map(|node| node.as_text())
//         .map(|text| text.text.as_ref())
//         .collect::<Vec<&str>>()
//         .join("")
//         .trim()
//         .to_string()
// }

fn map_select(el: ElementRef, result: &mut Vec<Halogen2>) {
    let name = el.value().attr("data-name").unwrap_or("na");
    if let Some(span_el) = el
        .children()
        .filter_map(ElementRef::wrap)
        .find(|elx| elx.value().name() == "span")
    {
        let span_class = match span_el.attr("class") {
            None => "",
            Some(clazz) => clazz,
        };

        let label_selector = Selector::parse("label").unwrap();
        let sel_selector = Selector::parse("select").unwrap();

        let label_el = span_el.select(&label_selector).next();
        let sel_el = span_el.select(&sel_selector).next();

        if let Some(label) = label_el {
            let label_class = match label.attr("class") {
                None => "",
                Some(clazz) => clazz,
            };
            //let label_text: String = label.text().collect::<String>().trim().to_string();

            let label_text = label
                .children()
                .filter_map(|node| {
                    if let Node::Text(tx) = node.value() {
                        Some(tx.text.to_string())
                    } else {
                        None
                    }
                })
                .collect::<String>()
                .trim()
                .to_string();

            if let Some(sel) = sel_el {
                let sel_class = match sel.attr("class") {
                    None => "",
                    Some(clazz) => clazz,
                };
                let evt_val = match sel.attr("onchange") {
                    None => "",
                    Some(evt) => evt,
                };
                // let options = sel
                //     .child_elements()
                //     .filter_map(|opt| Some(OptionData::new("ax", "bx")))
                //     .collect::<Vec<OptionData>>();

                let options: Vec<OptionData> = sel
                    .child_elements() // 1. Use child_elements() to get ElementRef directly
                    .filter_map(|opt| {
                        // 2. Extract actual data from 'opt' (e.g., text or attributes)
                        let text = opt.text().collect::<String>();
                        let attr = opt.attr("value").unwrap_or("");

                        // Return None if you want to skip specific elements
                        if text.is_empty() {
                            return None;
                        }

                        Some(OptionData::new(attr, &text))
                    })
                    .collect::<Vec<_>>();
                //.collect(); // 3. Type inference usually works here, or use .collect::<Vec<_>>()

                let sel_obj = SelectData2::new(
                    name,
                    &label_text,
                    span_class,
                    label_class,
                    sel_class,
                    evt_val,
                    options,
                );
                result.push(Halogen2::Select(sel_obj));
            }
        }
    }
    // let option_selector = Selector::parse("option").unwrap();
    // for option in el.select(&option_selector) {
    //     let value = option.value();
    //     let text = option.text().collect::<Vec<_>>().join("");
    //     println!("Option Value: {:?}, Text: {}", value, text);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Result, bail};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_input() -> Result<()> {
        let html = r#"
        <purs-input data-name="ednum">
          <span class="form-group">
            <label class="ps-label ps-mr-1">Edition#
              <input type="text" class="form-control ps-input" onchange="EdNumChange"></label></span></purs-input>
        "#;

        let document = Html::parse_fragment(html);

        let result = parse_html(&document);

        assert_eq!(1, result.len());

        match result.first() {
            Some(Halogen2::Input(inp)) => {
                assert_eq!("ednum", inp.name);
                assert_eq!("Edition#", inp.title);
                assert_eq!("form-group", inp.span_class);
                assert_eq!("ps-label ps-mr-1", inp.label_class);
                assert_eq!("form-control ps-input", inp.input_class);
                assert_eq!(InputType::InputText, inp.input_type);
                assert_eq!("EdNumChange", inp.evt);
                Ok(())
            }
            _ => {
                bail!("No purs-input found!")
            }
        }
    }
    #[test]
    fn test_parse_select() -> Result<()> {
        let html = r#"
            <purs-select data-name="edtype">
              <span class="form-group">
                  <label class="ps-label ps-mr-1 edition">Edition Type
                      <select onchange="EdTypeChange" class="form-control ps-select">
                          <option value="-">-</option>
                          <option value="1">A</option>
                          <option value="2">B</option>
                          <option value="3">C</option>
                      </select>
                  </label>
              </span>
            </purs-select>
          "#;

        let document = Html::parse_fragment(html);

        let result = parse_html(&document);

        assert_eq!(1, result.len());
        match result.first() {
            Some(Halogen2::Select(sel)) => {
                assert_eq!("edtype", sel.name);
                assert_eq!("Edition Type", sel.title);
                assert_eq!("form-group", sel.span_class);
                assert_eq!("ps-label ps-mr-1 edition", sel.label_class);
                assert_eq!("form-control ps-select", sel.select_class);
                assert_eq!("EdTypeChange", sel.evt);
                assert_eq!(4, sel.options.len());
                // let fst: &OptionData = &sel.options[0];
                // assert_eq!("-", fst.text);
                // assert_eq!("-", fst.text);
                test_option(&sel.options, 0, "-", "-");
                test_option(&sel.options, 1, "A", "1");
                test_option(&sel.options, 2, "B", "2");
                test_option(&sel.options, 3, "C", "3");

                Ok(())
            }

            _ => {
                bail!("No purs-input found!")
            }
        }
    }
    fn test_option(opts: &Vec<OptionData>, index: usize, exp_text: &str, exp_value: &str) {
        let opt: &OptionData = &opts[index];
        assert_eq!(exp_text, opt.text);
        assert_eq!(exp_value, opt.value);
    }
    #[test]
    fn test_parse_switch() -> Result<()> {
        Ok(())
    }
}

//<label>LABEL<select><option value="A">-</option></select></label>
