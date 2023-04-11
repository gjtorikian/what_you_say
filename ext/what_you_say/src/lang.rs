#[magnus::wrap(class = "WhatYouSay::Lang")]
pub struct WhatYouSayLang {
    code: String,
    name: String,
    eng_name: String,
}

// this is safe as WhatYouSayLang does not contain any Ruby types
unsafe impl magnus::IntoValueFromNative for WhatYouSayLang {}

impl WhatYouSayLang {
    pub fn new(code: String, name: String, eng_name: String) -> WhatYouSayLang {
        WhatYouSayLang {
            code,
            name,
            eng_name,
        }
    }

    pub fn all() -> Vec<WhatYouSayLang> {
        whatlang::Lang::all()
            .iter()
            .map(|lang| WhatYouSayLang {
                code: lang.code().to_string(),
                name: lang.name().to_string(),
                eng_name: lang.eng_name().to_string(),
            })
            .collect::<Vec<WhatYouSayLang>>()
    }

    pub fn code(&self) -> &str {
        self.code.as_str()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn eng_name(&self) -> &str {
        self.eng_name.as_str()
    }

    pub fn inspect(&self) -> String {
        format!(
            "#<WhatYouSay::Lang code=\"{0}\" name=\"{1}\" eng_name=\"{2}\">",
            self.code, self.name, self.eng_name
        )
    }
}
