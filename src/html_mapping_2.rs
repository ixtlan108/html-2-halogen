use scraper::{ElementRef, Html, Selector};
use std::fs;

#[derive(Debug)]
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
    pub evt: String,
    pub title: String,
    pub span_class: String,
    pub label_class: String,
    pub input_class: String,
    pub input_type: InputType,
    pub name: String,
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
pub enum Halogen2 {
    Input(InputData2),
}

pub fn parse_html(html_file: &str) -> Vec<Halogen2> {
    let mut halogens: Vec<Halogen2> = Vec::new();

    let html_content = fs::read_to_string(html_file).expect("Failed to read file");

    let input_selector = Selector::parse("purs-input").unwrap();

    let doc = Html::parse_document(&html_content);

    for input_ref in doc.select(&input_selector) {
        map_input(input_ref, &mut halogens);
    }

    halogens
}

fn map_input(el: ElementRef, result: &mut Vec<Halogen2>) {
    // <purs-input data-name="ednum"><span class="form-group"><label class="ps-label ps-mr-1">Edition#<input type="text" class="form-control ps-input" oncange="FetchMeAPizza"></label></span></purs-input>

    let name = el.value().attr("data-name").unwrap_or("na");

    println!("purs-input: {}", name);

    if let Some(span_el) = el
        .children()
        .filter_map(ElementRef::wrap)
        .find(|elx| elx.value().name() == "span")
    {
        let span_class = match span_el.attr("class") {
            None => "",
            Some(clazz) => clazz,
        };
        println!("Span class: {}", span_class);
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
            println!("Label text: {}, class: {}", label_text, label_class);

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
                println!("Input: {:?}", inp_obj);
                result.push(Halogen2::Input(inp_obj));
            }
        }

        //for node in span_el.descendants() {}
    }
}
