# WhatYouSay

Quick and easy natural language detection wrapping the [Whatlang Rust crate](https://github.com/greyblake/whatlang-rs). Instantly identify the source language of a piece of text.

![What you say!!](https://user-images.githubusercontent.com/64050/224237944-ceb2570c-d544-474a-8c91-41433efdee43.png)

- Supports [69 languages](https://github.com/greyblake/whatlang-rs/blob/master/SUPPORTED_LANGUAGES.md) (nice!)
- Core library is written in Rust; this is a Ruby wrapper to it
- Lightweight, fast, and simple
- Recognizes not only a language, but also a script (Latin, Cyrillic, etc)
- Provides reliability information

## Installation

Install the gem and add to the application's Gemfile by executing:

    $ bundle add what_you_say

If bundler is not being used to manage dependencies, install the gem by executing:

    $ gem install what_you_say

## Usage

The method to call is `_?`. Why? Because. Pass in the text whose language you want to detect:

```ruby
require "what_you_say"

text = "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!"

result = WhatYouSay._?(text)

assert_equal("epo", result.lang.code)
assert_equal("Esperanto", result.lang.name)
assert_equal("Esperanto", result.lang.eng_name)
assert_equal("Latin", result.script)
assert_predicate(result, :reliable?)
assert_equal(1, result.confidence)
```

You also have to opportunity to `inspect` some output:

```ruby
text = "Եվ ահա ես ստանում եմ մի զանգ պատահական տղայից"
WhatYouSay._?(text).inspect
#=> #<WhatYouSay::Info lang=#<WhatYouSay::Lang code="hye" name="Հայերեն" eng_name="Armenian"> script="Armenian" reliable=true confidence=1>
```

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake compile test` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and merge that change into `main`.

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/gjtorikian/what_you_say.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
