use crate::lang::WhatYouSayLang;

#[magnus::wrap(class = "WhatYouSay::Info")]
pub struct WhatYouSayInfo {
    lang: WhatYouSayLang,
    script: String,
    reliable: bool,
    confidence: f64,
}

impl WhatYouSayInfo {
    pub fn new(
        lang: WhatYouSayLang,
        script: String,
        reliable: bool,
        confidence: f64,
    ) -> WhatYouSayInfo {
        WhatYouSayInfo {
            lang,
            script,
            reliable,
            confidence,
        }
    }

    pub fn lang(&self) -> WhatYouSayLang {
        WhatYouSayLang::new(
            self.lang.code().to_string(),
            self.lang.name().to_string(),
            self.lang.eng_name().to_string(),
        )
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

    pub fn inspect(&self) -> String {
        format!(
            "#<WhatYouSay::Info \
                lang={0} \
                script=\"{1}\" reliable={2} confidence={3}>",
            self.lang.inspect(),
            self.script,
            self.reliable,
            self.confidence
        )
    }
}
