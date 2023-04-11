extern crate core;

use lang::WhatYouSayLang;
use lingua::LanguageDetectorBuilder;

use magnus::{define_module, function, method, scan_args, Error, Module, Object, RHash, Value};

fn detect_text<'a>(args: &[Value]) -> Result<WhatYouSayLang, magnus::Error> {
    let args = scan_args::scan_args(args)?;
    let (rb_text,): (String,) = args.required;
    let _: () = args.optional;
    let _: () = args.splat;
    let _: () = args.trailing;
    let _: () = args.block;

    let kwargs =
        scan_args::get_kwargs::<_, (), (Option<RHash>,), ()>(args.keywords, &[], &["options"])?;
    let _rb_options = kwargs.optional;

    let mut binding = LanguageDetectorBuilder::from_all_languages();
    let builder = binding.with_preloaded_language_models();

    let detector = builder.build();

    match detector.detect_language_of(rb_text) {
        Some(lang) => {
            let lang = WhatYouSayLang::new(lang.iso_code_639_3().to_string(), lang.to_string());

            Ok(lang)
        }
        None => Ok(unknown_lang()),
    }
}

fn unknown_lang() -> WhatYouSayLang {
    WhatYouSayLang::new("???".to_string(), "Unknown".to_string())
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("WhatYouSay")?;

    module.define_module_function("detect", function!(detect_text, -1))?;

    let c_lang = module.define_class("Lang", Default::default())?;
    c_lang.define_singleton_method("all", function!(WhatYouSayLang::all, 0))?;
    c_lang.define_method("code", method!(WhatYouSayLang::code, 0))?;
    c_lang.define_method("eng_name", method!(WhatYouSayLang::eng_name, 0))?;
    c_lang.define_method("inspect", method!(WhatYouSayLang::inspect, 0))?;

    Ok(())
}

pub mod lang;
