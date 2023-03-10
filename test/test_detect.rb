# frozen_string_literal: true

require "test_helper"
require "json"

class TestWhatYouSayDetect < Minitest::Test
  def setup
    @text = "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!"
  end

  def test_that_it_detects
    result = WhatYouSay._?(@text)

    assert_equal("epo", result.lang.code)
    assert_equal("Esperanto", result.lang.name)
    assert_equal("Esperanto", result.lang.eng_name)
    assert_equal("Latin", result.script)
    assert_predicate(result, :reliable?)
    assert_equal(1, result.confidence)
  end

  def test_to_s_works
    result = WhatYouSay._?(@text)

    actual_lang = "#<WhatYouSay::Lang code=\"epo\" name=\"Esperanto\" eng_name=\"Esperanto\">"
    actual_info = "#<WhatYouSay::Info lang=#{actual_lang} script=\"Latin\" reliable=true confidence=1>"

    assert_equal(actual_info, result.to_s)
    assert_equal(actual_lang, result.lang.to_s)
  end

  def test_confidence_is_a_float
    result = WhatYouSay._?("asdadce")

    # 0.5 is random, just want to prove this is a float
    assert_operator(result.confidence, :<, 0.5)
    assert_operator(result.confidence, :>, 0)
    assert_equal(result.confidence.class, Float)
  end

  def test_with_hella_examples
    example_data = JSON.parse(File.read(File.join("test", "fixtures", "examples.json")))

    example_data.each_pair do |lang_code, text|
      detected_lang = WhatYouSay._?(text).lang

      assert_equal(lang_code, detected_lang.code)
    end
  end
end
