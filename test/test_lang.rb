# frozen_string_literal: true

require "test_helper"
require "json"

class TestWhatYouSayLang < Minitest::Test
  def test_that_it_gets_all_languages
    results = WhatYouSay::Lang.all

    assert_operator(results.size, :>=, 69)
    assert_match(/\S{3}/, results[42].code)
  end
end
