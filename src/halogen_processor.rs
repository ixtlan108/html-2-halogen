use crate::html_mapping::{ButtonData, DivData, Halogen, InputData, SelectData, SwitchData};
use crate::html_mapping_2::{
    Halogen2, InputData2, InputType, OptionData, SelectData2, SwitchData2,
};

use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

// enum HLType {
//     Parent,
//     FirstChild,
//     Child,
// }

#[derive(Debug)]
struct HalogenLine {
    indent: u8,
    line: String,
}

impl HalogenLine {
    fn new(indent: u8, line: String) -> Self {
        Self {
            indent: indent,
            line: line,
        }
    }
}

type HL = HalogenLine;

fn module_path_2_package(module: &str) -> String {
    String::from(module.replace("/", "."))
}

fn split_module_main(s: &str) -> Option<(&str, &str)> {
    let idx = s.rfind('.')?;
    Some((&s[..idx], &s[idx + 1..]))
}

fn main_action_to_import(main_action: &str) -> String {
    if let Some((module_path, act)) = split_module_main(main_action) {
        format!("import {} ({}(..))", module_path, act)
    } else {
        String::from("error")
    }

    // let parts: Vec<&str> = main_action.split('.').collect();

    // if parts.len() >= 2 {
    //     // Join all parts except the last one for the module path
    //     let module_path = parts[..parts.len() - 1].join(".");
    //     // Get the last part for the function/type name
    //     let item_name = parts.last().unwrap();

    //     format!("import {} ({}(..))", module_path, item_name)
    // } else {
    //     // Fallback for invalid input (e.g., "Rapanui")
    //     format!("import {} (..)", main_action)
    // }
}

fn write_prelude<W: Write>(f: &mut W, module: &str, main_action: &str) -> Result<()> {
    writeln!(f, "module {} where", module_path_2_package(module))?;
    writeln!(f)?;
    writeln!(f, "import Prelude")?;
    writeln!(f, "import Data.Maybe (Maybe(..))")?;
    writeln!(f, "import Data.Array ((:))")?;
    writeln!(f, "import DOM.HTML.Indexed.InputType (InputType(..))")?;
    writeln!(f, "import Halogen.HTML as HH")?;
    writeln!(f, "import Halogen.HTML.Events as HE")?;
    writeln!(f, "import Halogen.HTML.Properties as HP")?;
    writeln!(f, "import Halogen.HTML (HTML, ClassName(..), AttrName(..))")?;
    let ma = main_action_to_import(main_action);
    writeln!(f, "{}", ma)?;
    Ok(())
}

fn write_fn_def<W: Write>(f: &mut W) -> Result<()> {
    writeln!(f)?;
    writeln!(f, "view :: forall w. HTML w MainAction")?;
    writeln!(f, "view = ")?;
    Ok(())
}

fn write_lines<W: Write>(f: &mut W, lines: &Vec<HalogenLine>, indent: u8) -> Result<()> {
    for line in lines.iter() {
        let sum_indent = line.indent + indent;
        match sum_indent {
            0 => writeln!(f, "{}", line.line)?,
            1 => writeln!(f, "  {}", line.line)?,
            2 => writeln!(f, "    {}", line.line)?,
            3 => writeln!(f, "      {}", line.line)?,
            4 => writeln!(f, "        {}", line.line)?,
            5 => writeln!(f, "          {}", line.line)?,
            6 => writeln!(f, "            {}", line.line)?,
            7 => writeln!(f, "              {}", line.line)?,
            8 => writeln!(f, "                {}", line.line)?,
            9 => writeln!(f, "                  {}", line.line)?,
            10 => writeln!(f, "                   {}", line.line)?,
            _ => writeln!(f, "{}", "error")?,
        };
    }
    Ok(())
}

fn map_button(btn_data: &ButtonData, result: &mut Vec<HalogenLine>, is_first: bool, indent: u8) {
    // pwfShow :: forall w. HTML w MainAction
    // pwfShow =
    //   HH.button [HE.onClick (GeneratePwf false), HP.classes [ ClassName "btn ps-btn btn-outline-success ps-mt-auto ps-mr-1 ps-btn-250 "]] [HH.text "Show current PWF#"]
    let btn = if is_first {
        format!(
            "HH.button [HE.onClick {}, HP.classes [ ClassName \"{}\"]] [HH.text \"{}\"]",
            btn_data.evt, btn_data.clazz, btn_data.title
        )
    } else {
        format!(
            ", HH.button [HE.onClick {}, HP.classes [ ClassName \"{}\"]] [HH.text \"{}\"]",
            btn_data.evt, btn_data.clazz, btn_data.title
        )
    };
    //let btn_x = if is_first { btn } else { btn };
    let line = HL::new(indent, btn);
    result.push(line);
}

fn map_input(inp_data: &InputData, result: &mut Vec<HalogenLine>, is_first: bool, indent: u8) {
    let inp = if is_first {
        format!("{} Nothing", inp_data.name)
    } else {
        format!(", {} Nothing", inp_data.name)
    };
    let line = HL::new(indent, inp);
    result.push(line);
}

fn map_select(data: &SelectData, result: &mut Vec<HalogenLine>, is_first: bool, indent: u8) {
    let sel = if is_first {
        format!("{} \"-\"", data.name)
    } else {
        format!(", {} \"-\"", data.name)
    };
    let line = HL::new(indent, sel);
    result.push(line);
}

fn map_switch(data: &SwitchData, result: &mut Vec<HalogenLine>, is_first: bool, indent: u8) {
    let sel = if is_first {
        format!("{} true", data.name)
    } else {
        format!(", {} true", data.name)
    };
    let line = HL::new(indent, sel);
    result.push(line);
}

fn map_div(div_data: &DivData, result: &mut Vec<HalogenLine>, is_first: bool) {
    //let mut result: Vec<HalogenLine> = Vec::new();

    let dpt = div_data.depth;
    let hh_div = if is_first { "HH.div" } else { ", HH.div" };

    result.push(HL::new(0 + dpt, format!("{}", hh_div)));
    result.push(HL::new(
        1 + dpt,
        format!("[ HP.classes [ ClassName \"{}\" ]]", div_data.clazz),
    ));
    result.push(HL::new(1 + dpt, String::from("[")));
    if let Some((first, rest)) = div_data.children.split_first() {
        match first {
            Halogen::Div(data) => map_div(data, result, true),
            Halogen::Button(data) => map_button(data, result, true, dpt + 2),
            Halogen::Input(data) => map_input(data, result, true, dpt + 2),
            Halogen::Select(data) => map_select(data, result, true, dpt + 2),
            Halogen::Switch(data) => map_switch(data, result, true, dpt + 2),
            _ => {}
        }
        for child in rest.iter() {
            match child {
                Halogen::Div(data) => map_div(data, result, false),
                Halogen::Button(data) => map_button(data, result, false, dpt + 2),
                Halogen::Input(data) => map_input(data, result, false, dpt + 2),
                Halogen::Select(data) => map_select(data, result, false, dpt + 2),
                Halogen::Switch(data) => map_switch(data, result, false, dpt + 2),
                _ => {}
            }
        }
    }
    result.push(HL::new(1 + dpt, String::from("]")));
}

fn write_div<W: Write>(f: &mut W, div_data: &DivData, indent_level: u8) -> Result<()> {
    let mut lines: Vec<HalogenLine> = Vec::new();
    map_div(div_data, &mut lines, true);
    write_lines(f, &lines, indent_level)?;

    for hal in lines.iter() {
        println!("{:?}", hal);
    }

    Ok(())
}

//static NEW_LINE: LazyLock<HalogenLine> = LazyLock::new(|| HL::new(0, String::from("")));

fn map_input_2(data: &InputData2, result: &mut Vec<HalogenLine>) {
    let fn_def = match data.input_type {
        InputType::InputText => HL::new(
            0,
            format!(
                "{} :: forall w. Maybe String -> HTML w MainAction",
                data.name
            ),
        ),
    };
    result.push(HL::new(0, String::from("")));
    result.push(fn_def);
    result.push(HL::new(0, format!("{} val = ", data.name)));
    result.push(HL::new(
        1,
        format!(
            "HH.span [ HP.classes [ ClassName \"{}\" ]]",
            data.span_class
        ),
    ));
    result.push(HL::new(
        2,
        format!(
            "[ HH.label [ HP.classes [ ClassName \"{}\" ]]",
            data.label_class
        ),
    ));
    result.push(HL::new(3, format!("[ HH.text \"{}\",", data.title)));
    result.push(HL::new(4, String::from("case val of")));
    result.push(HL::new(5, String::from("Nothing ->")));
    result.push(HL::new(
        6,
        format!(
            "HH.input [HP.type_ InputText, HP.classes [ ClassName \"{}\" ]]",
            data.input_class
        ),
    ));
    result.push(HL::new(5, String::from("Just val1 ->")));
    result.push(HL::new(
        6,
        format!(
            "HH.input [HP.type_ InputText, HP.classes [ ClassName \"{}\" ], HP.value val1]",
            data.input_class
        ),
    ));
    result.push(HL::new(3, String::from("]")));
    result.push(HL::new(2, String::from("]")));
}

fn map_option(data: &OptionData, is_first: bool, result: &mut Vec<HalogenLine>) {
    let init_line = if is_first == true {
        String::from("HH.option")
    } else {
        String::from(", HH.option")
    };
    result.push(HL::new(7, init_line));
    result.push(HL::new(8, format!("[ HP.value \"{}\"", data.value)));
    result.push(HL::new(
        8,
        format!(", HP.selected (\"{}\" == selected)]", data.value),
    ));
    result.push(HL::new(8, format!("[ HH.text \"{}\"]", data.text)));
}
fn map_select_2(data: &SelectData2, result: &mut Vec<HalogenLine>) {
    result.push(HL::new(0, String::from("")));
    result.push(HL::new(
        0,
        format!("{}:: forall w. String -> HTML w MainAction", data.name),
    ));
    result.push(HL::new(0, format!("{} selected =", data.name)));
    //   HH.span [ HP.classes [ ClassName "form-group" ]]
    result.push(HL::new(
        1,
        format!(
            "HH.span [ HP.classes [ ClassName \"{}\" ]]",
            data.span_class
        ),
    ));
    result.push(HL::new(
        2,
        format!(
            "[ HH.label [ HP.classes [ ClassName \"{}\" ]]",
            data.label_class
        ),
    ));
    result.push(HL::new(3, String::from("[ HH.text \"Risc\",")));
    result.push(HL::new(4, String::from("let")));
    result.push(HL::new(5, String::from("opts = ")));
    result.push(HL::new(6, String::from("[")));

    if let Some((first, rest)) = data.options.split_first() {
        map_option(&first, true, result);
        for item in rest.iter() {
            map_option(&item, false, result);
        }
    };

    result.push(HL::new(6, String::from("]")));
    result.push(HL::new(4, String::from("in")));
    result.push(HL::new(4, String::from("HH.select")));
    result.push(HL::new(
        5,
        format!("[ HP.classes [ ClassName \"{}\" ]", data.select_class),
    ));
    result.push(HL::new(5, format!(", HE.onValueChange {}", data.evt)));
    result.push(HL::new(5, String::from(", HP.disabled false ]")));
    result.push(HL::new(5, String::from("opts")));
    result.push(HL::new(5, String::from("]")));
    result.push(HL::new(4, String::from("]")));
}

fn map_switch_2(data: &SwitchData2, result: &mut Vec<HalogenLine>) {
    //*
    result.push(HL::new(0, String::from("")));
    result.push(HL::new(
        0,
        format!("{}:: forall w. Boolean -> HTML w MainAction", data.name),
    ));
    result.push(HL::new(0, format!("{} checked =", data.name)));
    result.push(HL::new(1, String::from("let")));
    result.push(HL::new(2, format!("btnId = \"{}\"", data.input_id)));
    result.push(HL::new(1, String::from("in")));
    result.push(HL::new(1, String::from("HH.div")));
    result.push(HL::new(
        2,
        format!("[ HP.classes [ ClassName \"{}\" ]]", data.div_class),
    ));
    result.push(HL::new(2, String::from("[ HH.input")));
    result.push(HL::new(3, String::from("[ HP.type_ InputCheckbox")));
    result.push(HL::new(
        3,
        format!(", HP.classes [ ClassName \"{}\" ]", data.input_class),
    ));
    result.push(HL::new(3, String::from(", HP.id btnId")));
    result.push(HL::new(3, String::from(", HP.disabled false")));
    result.push(HL::new(3, format!(", HE.onChecked {}", data.evt)));
    result.push(HL::new(3, String::from(", HP.checked checked")));
    result.push(HL::new(3, String::from("]")));
    result.push(HL::new(2, String::from(", HH.label")));
    result.push(HL::new(
        3,
        String::from("[ HP.attr (AttrName \"for\") btnId"),
    ));
    result.push(HL::new(
        3,
        format!(", HP.classes [ ClassName \"{}\"]", data.label_class),
    ));
    result.push(HL::new(3, String::from("]")));

    result.push(HL::new(3, format!("[ HH.text \"{}\" ]", data.title)));
    result.push(HL::new(2, String::from("]")));
    //*/
}

fn write_halogen_2<W: Write>(f: &mut W, items: &Vec<Halogen2>) -> Result<()> {
    let mut lines: Vec<HalogenLine> = Vec::new();

    for item in items.iter() {
        match item {
            Halogen2::Input(data) => map_input_2(data, &mut lines),
            Halogen2::Select(data) => map_select_2(data, &mut lines),
            Halogen2::Switch(data) => map_switch_2(data, &mut lines),
        };
    }

    write_lines(f, &lines, 0)?;

    Ok(())
}

pub fn generate(
    items: &Vec<Halogen>,
    items2: &Vec<Halogen2>,
    output: &Path,
    module: &str,
    main_action: &str,
) -> Result<()> {
    let mut out = File::create(output)?;

    write_prelude(&mut out, module, main_action)?;

    write_halogen_2(&mut out, &items2)?;

    if !items.is_empty() {
        write_fn_def(&mut out)?;
    }
    for item in items.iter() {
        match item {
            Halogen::Div(data) => write_div(&mut out, data, 1)?,
            _ => {}
        };
    }

    Ok(())
}

/*
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use scraper::{ElementRef, Html, Selector};

/// Translate an HTML file into a PureScript Halogen render function.
#[derive(Parser, Debug)]
#[command(
    name = "html2purs",
    about = "Translate an HTML file into a PureScript Halogen .purs file"
)]
struct Args {
    /// Path to the input HTML file
    input: PathBuf,

    /// Path to the output .purs file (defaults to <input>.purs)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// PureScript module name to emit
    #[arg(short, long, default_value = "Component.Generated")]
    module: String,

    /// Name of the generated render function
    #[arg(short = 'f', long, default_value = "render")]
    function_name: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let html_source = fs::read_to_string(&args.input)
        .with_context(|| format!("failed to read {}", args.input.display()))?;
    let document = Html::parse_document(&html_source);

    // Prefer the <body> contents; fall back to the whole document (e.g. for
    // HTML fragments that don't have a <html>/<body> wrapper at all).
    let body_selector = Selector::parse("body").unwrap();
    let root = document
        .select(&body_selector)
        .next()
        .unwrap_or_else(|| document.root_element());

    let output_source = render_module(&args.module, &args.function_name, root);

    let output_path = args
        .output
        .unwrap_or_else(|| args.input.with_extension("purs"));
    fs::write(&output_path, output_source)
        .with_context(|| format!("failed to write {}", output_path.display()))?;

    println!("Wrote {}", output_path.display());
    Ok(())
}

/// Build the full .purs file: module header, imports, and the render function.
fn render_module(module: &str, function_name: &str, root: ElementRef) -> String {
    let body_children = render_child_blocks(root, 1);

    let render_lines: Vec<String> = if body_children.is_empty() {
        vec![format!("{}HH.text \"\"", pad(1))]
    } else if body_children.len() == 1 {
        body_children.into_iter().next().unwrap()
    } else {
        // Multiple top-level nodes: wrap them in a plain HH.div_ so the
        // function still returns exactly one HTML value.
        let mut lines = vec![format!("{}HH.div_", pad(1))];
        lines.extend(format_array(body_children, 2));
        lines
    };

    let mut out = String::new();
    out.push_str(&format!("module {} ({}) where\n\n", module, function_name));
    out.push_str("import Prelude\n");
    out.push_str("import Halogen.HTML as HH\n");
    out.push_str("import Halogen.HTML.Properties as HP\n\n");
    out.push_str(&format!("{} :: forall w i. HH.HTML w i\n", function_name));
    out.push_str(&format!("{} =\n", function_name));
    out.push_str(&render_lines.join("\n"));
    out.push('\n');
    out
}

/// Render a single element (and everything under it) as a block of lines.
/// `level` is the indentation level of the element's own head line
/// (e.g. `HH.div`); nested arrays get rendered one level deeper.
fn render_element_lines(el: ElementRef, level: usize) -> Vec<String> {
    let tag_name = el.value().name();

    let head = match tag_to_hh(tag_name) {
        Some(hh_fn) => hh_fn.to_string(),
        None => format!("HH.element (HH.ElemName \"{}\")", tag_name),
    };

    let mut lines = vec![format!("{}{}", pad(level), head)];

    let prop_blocks = render_prop_blocks(el);
    lines.extend(format_array(prop_blocks, level + 1));

    let child_blocks = render_child_blocks(el, level + 1);
    lines.extend(format_array(child_blocks, level + 1));

    lines
}

/// Render an element's attributes as a list of one-line "blocks", each
/// being a single `HP.foo ...` expression. `class` is merged into a single
/// `HP.class_` / `HP.classes` entry.
fn render_prop_blocks(el: ElementRef) -> Vec<Vec<String>> {
    let mut props: Vec<String> = Vec::new();
    let mut classes: Vec<String> = Vec::new();

    for (name, value) in el.value().attrs() {
        match name {
            "class" => classes = value.split_whitespace().map(str::to_string).collect(),
            "id" => props.push(format!("HP.id \"{}\"", escape_ps_string(value))),
            "href" => props.push(format!("HP.href \"{}\"", escape_ps_string(value))),
            "src" => props.push(format!("HP.src \"{}\"", escape_ps_string(value))),
            "alt" => props.push(format!("HP.alt \"{}\"", escape_ps_string(value))),
            "title" => props.push(format!("HP.title \"{}\"", escape_ps_string(value))),
            "placeholder" => props.push(format!("HP.placeholder \"{}\"", escape_ps_string(value))),
            "value" => props.push(format!("HP.value \"{}\"", escape_ps_string(value))),
            "name" => props.push(format!("HP.name \"{}\"", escape_ps_string(value))),
            "for" => props.push(format!("HP.for \"{}\"", escape_ps_string(value))),
            // Anything else (data-*, type, role, aria-*, ...) falls back to
            // a raw attribute, which always compiles regardless of Halogen
            // version or whether a typed helper exists for it.
            other => props.push(format!(
                "HP.attr (HH.AttrName \"{}\") \"{}\"",
                other,
                escape_ps_string(value)
            )),
        }
    }

    // Insert the class property first, to roughly mirror source order.
    if classes.len() == 1 {
        props.insert(
            0,
            format!(
                "HP.class_ (HH.ClassName \"{}\")",
                escape_ps_string(&classes[0])
            ),
        );
    } else if classes.len() > 1 {
        let class_list = classes
            .iter()
            .map(|c| format!("HH.ClassName \"{}\"", escape_ps_string(c)))
            .collect::<Vec<_>>()
            .join(", ");
        props.insert(0, format!("HP.classes [ {} ]", class_list));
    }

    props.into_iter().map(|p| vec![p]).collect()
}

/// Render an element's children (elements and non-blank text nodes) as
/// blocks. Each block is either a single-line `HH.text "..."` or a full
/// multi-line `render_element_lines` result.
fn render_child_blocks(el: ElementRef, child_level: usize) -> Vec<Vec<String>> {
    let mut blocks = Vec::new();

    for child in el.children() {
        if let Some(child_el) = ElementRef::wrap(child) {
            // Skip elements that never carry visible content.
            if matches!(child_el.value().name(), "script" | "style") {
                continue;
            }
            blocks.push(render_element_lines(child_el, child_level));
        } else if let Some(text) = child.value().as_text() {
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                blocks.push(vec![format!("HH.text \"{}\"", escape_ps_string(trimmed))]);
            }
        }
        // Comments, doctypes, etc. are silently dropped.
    }

    blocks
}

/// Format a list of item blocks as a PureScript array literal:
///
/// ```text
/// [ item-one
/// , item-two
/// ]
/// ```
///
/// Each block's own first line may carry arbitrary leading whitespace (or
/// none) — it gets stripped and replaced with the correct marker and
/// indentation for this array. Later lines in a block are left untouched,
/// since PureScript doesn't require bracketed expressions to align (layout
/// rules only apply to `do`/`where`/`let`/`of` blocks).
fn format_array(item_blocks: Vec<Vec<String>>, level: usize) -> Vec<String> {
    if item_blocks.is_empty() {
        return vec![format!("{}[]", pad(level))];
    }

    let mut out = Vec::new();
    for (i, mut block) in item_blocks.into_iter().enumerate() {
        let marker = if i == 0 { "[ " } else { ", " };
        if let Some(first) = block.first_mut() {
            let trimmed = first.trim_start();
            *first = format!("{}{}{}", pad(level), marker, trimmed);
        }
        out.extend(block);
    }
    out.push(format!("{}]", pad(level)));
    out
}

fn pad(level: usize) -> String {
    "  ".repeat(level)
}

/// Escape a string for use inside a PureScript string literal.
fn escape_ps_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            other => out.push(other),
        }
    }
    out
}

/// Map an HTML tag name to its corresponding `Halogen.HTML` smart
/// constructor. Anything not covered here falls back to `HH.element` in
/// the caller, so unknown/custom tags never fail to translate.
fn tag_to_hh(tag: &str) -> Option<&'static str> {
    Some(match tag {
        "div" => "HH.div",
        "span" => "HH.span",
        "p" => "HH.p",
        "a" => "HH.a",
        "img" => "HH.img",
        "button" => "HH.button",
        "input" => "HH.input",
        "form" => "HH.form",
        "label" => "HH.label",
        "ul" => "HH.ul",
        "ol" => "HH.ol",
        "li" => "HH.li",
        "table" => "HH.table",
        "thead" => "HH.thead",
        "tbody" => "HH.tbody",
        "tfoot" => "HH.tfoot",
        "tr" => "HH.tr",
        "td" => "HH.td",
        "th" => "HH.th",
        "h1" => "HH.h1",
        "h2" => "HH.h2",
        "h3" => "HH.h3",
        "h4" => "HH.h4",
        "h5" => "HH.h5",
        "h6" => "HH.h6",
        "nav" => "HH.nav",
        "header" => "HH.header",
        "footer" => "HH.footer",
        "section" => "HH.section",
        "article" => "HH.article",
        "aside" => "HH.aside",
        "main" => "HH.main_",
        "br" => "HH.br",
        "hr" => "HH.hr",
        "strong" => "HH.strong",
        "em" => "HH.em",
        "b" => "HH.b",
        "i" => "HH.i",
        "u" => "HH.u",
        "small" => "HH.small",
        "pre" => "HH.pre",
        "code" => "HH.code",
        "blockquote" => "HH.blockquote",
        "textarea" => "HH.textarea",
        "select" => "HH.select",
        "option" => "HH.option",
        "fieldset" => "HH.fieldset",
        "legend" => "HH.legend",
        _ => return None,
    })
}
*/

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
