extern crate core;

use std::str::FromStr;

use lang::WhatYouSayLang;
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};

use magnus::{
    define_class, exception, function, method, scan_args, Error, Module, Object, RArray, Value,
};

#[magnus::wrap(class = "WhatYouSay")]
struct WhatYouSay {
    detector: LanguageDetector,
}

impl WhatYouSay {
    fn new(args: &[Value]) -> Result<Self, magnus::Error> {
        let args = scan_args::scan_args(args)?;
        let _: () = args.required;
        let _: () = args.optional;
        let _: () = args.splat;
        let _: () = args.trailing;
        let _: () = args.block;

        let kwargs = scan_args::get_kwargs::<_, (), (Option<RArray>,), ()>(
            args.keywords,
            &[],
            &["allowlist"],
        )?;
        let (rb_allowlist,) = kwargs.optional;

        let mut binding = match rb_allowlist {
            Some(languages) => {
                let mut allowed_languages = vec![];
                for allowed in languages.each() {
                    let allowed = match allowed {
                        Ok(allowed) => allowed.to_string(),
                        Err(_) => {
                            return Err(magnus::Error::new(
                                exception::runtime_error(),
                                format!("{allowed:?}"),
                            ))
                        }
                    };

                    // if !Ok, it maeans the language could not be found
                    if let Ok(language) = Language::from_str(&allowed) {
                        allowed_languages.push(language)
                    }
                }
                LanguageDetectorBuilder::from_languages(&allowed_languages)
            }
            None => LanguageDetectorBuilder::from_all_languages(),
        };

        let builder = binding.with_preloaded_language_models();

        let detector = builder.build();

        Ok(WhatYouSay { detector })
    }

    pub fn detect_text(&self, rb_text: String) -> Result<WhatYouSayLang, magnus::Error> {
        match self.detector.detect_language_of(rb_text) {
            Some(lang) => {
                let result = WhatYouSayLang::new(Some(lang));

                Ok(result)
            }
            None => Ok(WhatYouSayLang::new(None)),
        }
    }
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let c_whatyousay = define_class("WhatYouSay", Default::default())?;

    c_whatyousay.define_singleton_method("new", function!(WhatYouSay::new, -1))?;
    c_whatyousay.define_method("detect_text", method!(WhatYouSay::detect_text, 1))?;

    let c_lang = c_whatyousay.define_class("Lang", Default::default())?;
    c_lang.define_singleton_method("all", function!(WhatYouSayLang::all, 0))?;
    c_lang.define_method("code", method!(WhatYouSayLang::code, 0))?;
    c_lang.define_method("eng_name", method!(WhatYouSayLang::eng_name, 0))?;
    c_lang.define_method("inspect", method!(WhatYouSayLang::inspect, 0))?;

    Ok(())
}

pub mod lang;
