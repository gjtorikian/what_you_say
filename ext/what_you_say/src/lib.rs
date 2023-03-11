extern crate core;

use info::WhatYouSayInfo;
use lang::WhatYouSayLang;
use whatlang::detect as whatlang_detect;

use magnus::{
    define_module, exception, function, method, scan_args, Error, Module, Object, RHash, Value,
};

fn detect_text<'a>(args: &[Value]) -> Result<WhatYouSayInfo, magnus::Error> {
    let args = scan_args::scan_args(args)?;
    let (rb_text,): (String,) = args.required;
    let _: () = args.optional;
    let _: () = args.splat;
    let _: () = args.trailing;
    let _: () = args.block;

    let kwargs =
        scan_args::get_kwargs::<_, (), (Option<RHash>,), ()>(args.keywords, &[], &["options"])?;
    let _rb_options = kwargs.optional;

    match whatlang_detect(&rb_text) {
        Some(info) => {
            let lang = WhatYouSayLang::new(
                info.lang().code().to_string(),
                info.lang().name().to_string(),
                info.lang().eng_name().to_string(),
            );

            let script = info.script().to_string();
            let reliable = info.is_reliable();
            let confidence = info.confidence();
            let what_you_say_info = WhatYouSayInfo::new(lang, script, reliable, confidence);
            Ok(what_you_say_info)
        }
        None => Err(Error::new(
            exception::arg_error(),
            format!("unable to identify text: {rb_text}"),
        )),
    }
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("WhatYouSay")?;

    module.define_module_function("detect", function!(detect_text, -1))?;

    let c_info = module.define_class("Info", Default::default())?;

    c_info.define_method("lang", method!(WhatYouSayInfo::lang, 0))?;
    c_info.define_method("script", method!(WhatYouSayInfo::script, 0))?;
    c_info.define_method("reliable?", method!(WhatYouSayInfo::reliable, 0))?;
    c_info.define_method("confidence", method!(WhatYouSayInfo::confidence, 0))?;
    c_info.define_method("to_s", method!(WhatYouSayInfo::to_s, 0))?;

    let c_lang = module.define_class("Lang", Default::default())?;
    c_lang.define_singleton_method("all", function!(WhatYouSayLang::all, 0))?;
    c_lang.define_method("code", method!(WhatYouSayLang::code, 0))?;
    c_lang.define_method("name", method!(WhatYouSayLang::name, 0))?;
    c_lang.define_method("eng_name", method!(WhatYouSayLang::eng_name, 0))?;
    c_lang.define_method("to_s", method!(WhatYouSayLang::to_s, 0))?;

    Ok(())
}

pub mod info;
pub mod lang;
