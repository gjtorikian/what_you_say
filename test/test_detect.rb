# frozen_string_literal: true

require "test_helper"
require "json"

class TestWhatYouSayDetect < Minitest::Test
  def setup
    @esperanto = "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!"
    @japanese = "This is some japanese text 日本が大好きです。"
  end

  def test_that_it_detects
    result = WhatYouSay._?(@esperanto)

    assert_equal("epo", result.code)
    assert_equal("esperanto", result.eng_name)
  end

  def test_inspect_works
    result = WhatYouSay._?(@esperanto)

    actual_lang = "#<WhatYouSay::Lang code=\"epo\" eng_name=\"esperanto\">"

    assert_equal(actual_lang, result.inspect)
  end

  def test_with_hella_examples
    example_data = JSON.parse(File.read(File.join("test", "fixtures", "examples.json")))

    example_data.each_pair do |lang_code, text|
      detected_lang = WhatYouSay._?(text)

      assert_equal(lang_code, detected_lang.code)
    end
  end

  def test_detects_japanese
    result = WhatYouSay._?(@japanese)

    assert_equal("jpn", result.code)
  end
end
