extern crate rust_stemmers;
use rust_stemmers::{Algorithm, Stemmer};
use rustler::NifTaggedEnum;

#[derive(NifTaggedEnum, Clone, Debug)]
pub enum LanguageAlgorithm {
    Arabic,
    Armenian,
    Danish,
    Dutch,
    English,
    Finnish,
    French,
    German,
    Greek,
    Hungarian,
    Italian,
    Norwegian,
    Portuguese,
    Romanian,
    Russian,
    Spanish,
    Swedish,
    Tamil,
    Turkish,
}
#[derive(NifTaggedEnum, Debug)]
pub enum ParamsPub {
    Language(LanguageAlgorithm),
}
#[derive(Debug)]
struct ParamsPriv {
    language: Algorithm,
}
fn get_options<'a>(options: Vec<ParamsPub>) -> ParamsPriv {
    let mut opts = ParamsPriv {
        language: Algorithm::English,
    };
    options.iter().for_each(|option| match option {
        ParamsPub::Language(val) => {
            opts.language = match *val {
                LanguageAlgorithm::Arabic => Algorithm::Arabic,
                LanguageAlgorithm::Danish => Algorithm::Danish,
                LanguageAlgorithm::Dutch => Algorithm::Dutch,
                LanguageAlgorithm::English => Algorithm::English,
                LanguageAlgorithm::Finnish => Algorithm::Finnish,
                LanguageAlgorithm::French => Algorithm::French,
                LanguageAlgorithm::German => Algorithm::German,
                LanguageAlgorithm::Greek => Algorithm::Greek,
                LanguageAlgorithm::Hungarian => Algorithm::Hungarian,
                LanguageAlgorithm::Italian => Algorithm::Italian,
                LanguageAlgorithm::Norwegian => Algorithm::Norwegian,
                LanguageAlgorithm::Portuguese => Algorithm::Portuguese,
                LanguageAlgorithm::Romanian => Algorithm::Romanian,
                LanguageAlgorithm::Russian => Algorithm::Russian,
                LanguageAlgorithm::Spanish => Algorithm::Spanish,
                LanguageAlgorithm::Swedish => Algorithm::Swedish,
                LanguageAlgorithm::Tamil => Algorithm::Tamil,
                LanguageAlgorithm::Turkish => Algorithm::Turkish,
                _ => Algorithm::English,
            }
        }
    });
    opts
}
#[rustler::nif]
fn stemmer(text: &str, options: Vec<ParamsPub>) -> String {
    let opts = get_options(options);
    let en_stemmer = Stemmer::create(opts.language);
    en_stemmer.stem(text).to_string()
}

rustler::init!("Elixir.ExStemmers.Native", [stemmer]);
