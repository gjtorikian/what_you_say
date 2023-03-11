# frozen_string_literal: true

require "test_helper"
require "json"

class TestWhatYouSayLang < Minitest::Test
  def test_that_it_gets_all_languages
    results = WhatYouSay::Lang.all

    assert 69 >= results.size
    assert_equal "tam", results[42].code
    assert_equal "Tamil", results[42].eng_name
  end
end
