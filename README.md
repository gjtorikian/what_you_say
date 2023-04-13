# WhatYouSay

Quick and easy natural language detection wrapping the [lingua-rs Rust crate](https://github.com/pemistahl/lingua-rs). Instantly identify the source language of a piece of text.

![What you say!!](https://user-images.githubusercontent.com/64050/224237944-ceb2570c-d544-474a-8c91-41433efdee43.png)

- Supports [75+ languages](https://github.com/pemistahl/lingua-rs/tree/main#3-which-languages-are-supported)
- Core library is written in Rust; this is a Ruby wrapper to it
- Lightweight, fast, and simple

## Installation

Install the gem and add to the application's Gemfile by executing:

    $ bundle add what_you_say

If bundler is not being used to manage dependencies, install the gem by executing:

    $ gem install what_you_say

## Usage

The method to call is `detect_language`.

Pass in the text whose language you want to detect:

```ruby
require "what_you_say"

text = "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!"

result = WhatYouSay.new.detect_language(text)

assert_equal("epo", result.lang.code)
assert_equal("esperanto", result.lang.eng_name)
```

You also have to opportunity to `inspect` some output:

```ruby
text = "Եվ ահա ես ստանում եմ մի զանգ պատահական տղայից"
WhatYouSay.new.detect_language(text).inspect
#=> #<WhatYouSay::Lang code="hye" eng_name="armenian">
```

Not everything in life is perfect, and neither is this lib. Sometimes language detection will be wildly mistaken. You
can attempt to correct this by passing in an `allowlist` of supported languages:

```ruby
text = "สวัสดี Rágis hello"
result = WhatYouSay.new.detect_language(text)

assert_equal("spanish", result.eng_name)

result = WhatYouSay.new(allowlist: ["English", "Thai"]).detect_language(text)

assert_equal("eng", result.code)
```

If a language truly cannot be detected, the `Unknown` language type is returned:

```ruby
text = "日本語"

result = WhatYouSay.new(allowlist: ["English", "Thai"]).detect_language(text)

assert_equal("???", result.code)
assert_equal("unknown", result.eng_name)
```

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake compile test` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and merge that change into `main`.

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/gjtorikian/what_you_say.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
