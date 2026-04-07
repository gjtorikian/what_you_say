# frozen_string_literal: true

require "test_helper"

class TestWhatYouSayDetectWithConfidence < Minitest::Test
  def setup
    @detector = WhatYouSay.new
    @english_text = "languages are awesome"
    @japanese_text = "This is some japanese text 日本が大好きです。"
  end

  def test_most_likely_language_has_highest_confidence
    result = @detector.detect_language_with_confidence(@english_text)

    top_lang, top_confidence = result.first

    assert_equal("eng", top_lang.code)

    result[1..].each do |_, confidence|
      assert_operator(top_confidence, :>, confidence)
    end
  end

  def test_results_sorted_by_confidence_descending
    result = @detector.detect_language_with_confidence(@english_text)

    confidences = result.map { |_, c| c }

    assert_equal(confidences.sort.reverse, confidences)
  end

  def test_with_allowlist_scopes_results
    detector = WhatYouSay.new(allowlist: ["English", "French", "German", "Spanish"])
    result = detector.detect_language_with_confidence("languages are awesome")

    assert_equal(4, result.length)
    codes = result.map { |lang, _| lang.code }

    assert_includes(codes, "eng")
    refute_includes(codes, "jpn")
  end

  def test_detects_japanese_with_confidence
    result = @detector.detect_language_with_confidence(@japanese_text)

    top_lang, _ = result.first

    assert_equal("jpn", top_lang.code)
  end

  def test_raises_for_non_string
    assert_raises(TypeError) { @detector.detect_language_with_confidence(42) }
  end

  def test_raises_for_non_utf8
    text = "hello".encode("ISO-8859-1")

    assert_raises(TypeError) { @detector.detect_language_with_confidence(text) }
  end
end
