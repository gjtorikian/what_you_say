use lingua::Language;

#[magnus::wrap(class = "WhatYouSay::Lang")]
pub struct WhatYouSayLang {
    code: String,
    eng_name: String,
}

// this is safe as WhatYouSayLang does not contain any Ruby types
unsafe impl magnus::IntoValueFromNative for WhatYouSayLang {}

impl WhatYouSayLang {
    pub fn new(code: String, eng_name: String) -> WhatYouSayLang {
        WhatYouSayLang { code, eng_name }
    }
    pub fn all() -> Vec<WhatYouSayLang> {
        Language::all()
            .into_iter()
            .map(|lang| WhatYouSayLang::new(lang.iso_code_639_3().to_string(), lang.to_string()))
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
