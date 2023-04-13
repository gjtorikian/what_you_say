# frozen_string_literal: true

require "test_helper"
require "json"

class TestWhatYouSayDetect < Minitest::Test
  def setup
    @esperanto = "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!"
    @japanese = "This is some japanese text 日本が大好きです。"
  end

  def test_that_it_detects
    result = WhatYouSay.new.detect_language(@esperanto)

    assert_equal("epo", result.code)
    assert_equal("esperanto", result.eng_name)
  end

  def test_inspect_works
    result = WhatYouSay.new.detect_language(@esperanto)

    actual_lang = "#<WhatYouSay::Lang code=\"epo\" eng_name=\"esperanto\">"

    assert_equal(actual_lang, result.inspect)
  end

  def test_with_hella_examples
    example_data = JSON.parse(File.read(File.join("test", "fixtures", "examples.json")))

    example_data.each_pair do |lang_code, text|
      detected_lang = WhatYouSay.new.detect_language(text)

      assert_equal(lang_code, detected_lang.code)
    end
  end

  def test_detects_japanese
    result = WhatYouSay.new.detect_language(@japanese)

    assert_equal("jpn", result.code)
  end

  def test_works_with_allowlist
    text = "สวัสดี Rágis hello"
    result = WhatYouSay.new.detect_language(text)

    assert_equal("spanish", result.eng_name)

    result = WhatYouSay.new(allowlist: ["English", "Thai"]).detect_language(text)

    assert_equal("eng", result.code)
  end

  def test_returns_unknown_language
    text = "日本語"

    result = WhatYouSay.new(allowlist: ["English", "Thai"]).detect_language(text)

    assert_equal("???", result.code)
  end
end
