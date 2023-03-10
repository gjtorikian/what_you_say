# frozen_string_literal: true

begin
  # native precompiled gems package shared libraries in <gem_dir>/lib/what_you_say/<ruby_version>
  # load the precompiled extension file
  ruby_version = /\d+\.\d+/.match(RUBY_VERSION)
  require_relative "#{ruby_version}/what_you_say"
rescue LoadError
  # fall back to the extension compiled upon installation.
  # use "require" instead of "require_relative" because non-native gems will place C extension files
  # in Gem::BasicSpecification#extension_dir after compilation (during normal installation), which
  # is in $LOAD_PATH but not necessarily relative to this file (see nokogiri#2300)
  require "what_you_say/what_you_say"
end
