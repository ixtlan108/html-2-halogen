mod halogen_processor;
pub mod html_mapping;
pub mod html_mapping_2;

//use clap::{Arg, ArgAction, Command};
use clap::Parser;

//use crate::html_mapping::Halogen;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use yaml_rust2::YamlLoader;

//use crate::generate1::parse_html;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about)]
/// Html to Purescript Halogen
struct Args {
    /// Yaml Project File
    #[arg(long, required(true))]
    yaml: String,

    /// Print enter/leaving HTML nodes
    #[arg(short, long, default_value_t = false)]
    prn_enter_exit: bool,
}

#[derive(Debug)]
struct Config {
    main_action: String,
    module: String,
    html: String,
    src_path: String,
}

fn purs_file_name(src_path: &str, module: &str) -> PathBuf {
    let purs = format!("{}.purs", module);
    PathBuf::from(src_path).join(purs)
}
fn main() {
    let args = Args::parse();
    dbg!(args.clone());

    let cfg = parse_yaml(&args.yaml).unwrap();
    println!("{:?}", cfg);

    let output = purs_file_name(&cfg.src_path, &cfg.module);
    println!("{}", output.display());

    let mapped = html_mapping::parse_html(&cfg.html, args.prn_enter_exit);

    //let mapped: Vec<Halogen> = Vec::new();
    let mapped2 = html_mapping_2::parse_html_file(&cfg.html);

    let result =
        halogen_processor::generate(&mapped, &mapped2, &output, &cfg.module, &cfg.main_action);
}

fn parse_yaml(yaml: &str) -> Result<Config, Box<dyn Error>> {
    // let result = Config {
    //     main_action: String::from("CustomerCRM.Types.MainAction"),
    //     module: String::from("CustomerCRM/UI2"),
    //     html: String::from("tests/resources/01.html"),
    //     src_path: String::from("../../crm/PushwagnerCrm/purescript/crm/customer-crm/src/"),
    // };
    let yaml_content = fs::read_to_string(yaml).expect("Failed to read yaml file");

    let docs = YamlLoader::load_from_str(&yaml_content)?;
    let doc = &docs[0];
    let main_action = doc["main-action"].as_str().unwrap();
    let module = doc["module"].as_str().unwrap();
    let html = doc["html"].as_str().unwrap();
    let src_path = doc["src-path"].as_str().unwrap();

    let result = Config {
        main_action: String::from(main_action),
        module: String::from(module),
        html: String::from(html),
        src_path: String::from(src_path),
    };
    //let yaml_result = YamlLoader::load_from_str(yaml);
    Ok(result)
}

/*
let matches = Command::new("html2halogen")
    .version("1.0.0")
    .author("me")
    .about("Translate html to Purescript Halogen")
    .arg(
        Arg::new("html")
            .help("Html file to translate")
            .required(true)
            .num_args(1),
    )
    .arg(
        Arg::new("doit")
            .short('d')
            .long("doit")
            .action(ArgAction::SetTrue)
            .help("Whatever again"),
    )
    .get_matches();
println!("{:#?}", matches);
let html_file = matches.get_one::<String>("html");
println!("Html file: {:#?}", html_file);
let doit = matches.get_flag("doit");
println!("Do it?: {:#?}", doit);
*/
