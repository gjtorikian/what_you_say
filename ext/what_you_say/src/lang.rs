use lingua::Language;

#[magnus::wrap(class = "WhatYouSay::Lang")]
pub struct WhatYouSayLang {
    code: String,
    eng_name: String,
}

// this is safe as WhatYouSayLang does not contain any Ruby types
unsafe impl magnus::IntoValueFromNative for WhatYouSayLang {}

impl WhatYouSayLang {
    pub fn new(lang: Option<Language>) -> WhatYouSayLang {
        match lang {
            Some(lang) => WhatYouSayLang {
                code: lang.iso_code_639_3().to_string(),
                eng_name: lang.to_string(),
            },
            None => WhatYouSayLang {
                code: "???".to_string(),
                eng_name: "unknown".to_string(),
            },
        }
    }
    pub fn all() -> Vec<WhatYouSayLang> {
        Language::all()
            .into_iter()
            .map(|lang| WhatYouSayLang::new(Some(lang)))
            .collect::<Vec<_>>()
    }

    pub fn code(&self) -> &str {
        self.code.as_str()
    }

    pub fn eng_name(&self) -> &str {
        self.eng_name.as_str()
    }

    pub fn inspect(&self) -> String {
        format!(
            "#<WhatYouSay::Lang code=\"{0}\" eng_name=\"{1}\">",
            self.code, self.eng_name
        )
    }
}
