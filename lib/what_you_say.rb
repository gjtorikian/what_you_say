# frozen_string_literal: true

require_relative "what_you_say/extension"

require "what_you_say/lang"
require "what_you_say/version"

if ENV.fetch("DEBUG", false)
  require "debug"
end

class WhatYouSay
  def detect_language(text)
    raise TypeError, "text must be a String; got a #{text.class}!" unless text.is_a?(String)
    raise TypeError, "text must be UTF-8 encoded; got #{text.encoding}!" unless text.encoding.name == "UTF-8"

    detect_text(text)
  end
end
