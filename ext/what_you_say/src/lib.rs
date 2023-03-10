extern crate core;

use whatlang::detect as whatlang_detect;

use magnus::{define_module, exception, function, method, scan_args, Error, Module, RHash, Value};
#[magnus::wrap(class = "WhatYouSay::Info")]
pub struct WhatYouSayInfo {
    lang: WhatYouSayLang,
    script: String,
    reliable: bool,
    confidence: f64,
}

#[magnus::wrap(class = "WhatYouSay::Lang")]
pub struct WhatYouSayLang {
    code: String,
    name: String,
    eng_name: String,
}

impl WhatYouSayInfo {
    pub fn lang(&self) -> WhatYouSayLang {
        WhatYouSayLang {
            code: self.lang.code.clone(),
            name: self.lang.name.clone(),
            eng_name: self.lang.eng_name.clone(),
        }
    }

    pub fn script(&self) -> &str {
        self.script.as_str()
    }

    pub fn reliable(&self) -> bool {
        self.reliable
    }

    pub fn confidence(&self) -> f64 {
        self.confidence
    }

    pub fn to_s(&self) -> String {
        format!(
            "#<WhatYouSay::Info \
                lang={0} \
                script=\"{1}\" reliable={2} confidence={3}>",
            self.lang.to_s(),
            self.script,
            self.reliable,
            self.confidence
        )
    }
}

impl WhatYouSayLang {
    pub fn code(&self) -> &str {
        self.code.as_str()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn eng_name(&self) -> &str {
        self.eng_name.as_str()
    }

    pub fn to_s(&self) -> String {
        format!(
            "#<WhatYouSay::Lang code=\"{0}\" name=\"{1}\" eng_name=\"{2}\">",
            self.code, self.name, self.eng_name
        )
    }
}

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
            let lang = WhatYouSayLang {
                code: info.lang().code().to_string(),
                name: info.lang().name().to_string(),
                eng_name: info.lang().eng_name().to_string(),
            };
            let script = info.script().to_string();
            let reliable = info.is_reliable();
            let confidence = info.confidence();
            let what_you_say_info = WhatYouSayInfo {
                lang,
                script,
                reliable,
                confidence,
            };
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
    c_lang.define_method("code", method!(WhatYouSayLang::code, 0))?;
    c_lang.define_method("name", method!(WhatYouSayLang::name, 0))?;
    c_lang.define_method("eng_name", method!(WhatYouSayLang::eng_name, 0))?;
    c_lang.define_method("to_s", method!(WhatYouSayLang::to_s, 0))?;

    Ok(())
}
