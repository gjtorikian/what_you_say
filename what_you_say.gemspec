# frozen_string_literal: true

require_relative "lib/what_you_say/version"

Gem::Specification.new do |spec|
  spec.name = "what_you_say"
  spec.version = WhatYouSay::VERSION
  spec.authors = ["Garen J. Torikian"]
  spec.email = ["gjtorikian@users.noreply.github.com"]

  spec.summary = "Fast and lightweight language identification library. Written in Rust, wrapped in Ruby."
  spec.description = "Natural language detection with a focus on simplicity and performance. Currently wraps the lingua-rs Rust crate."
  spec.homepage = "https://github.com/gjtorikian/what_you_say"
  spec.license = "MIT"

  spec.required_ruby_version = ">= 3.2", "< 5"
  spec.required_rubygems_version = ">= 3.4"

  spec.files = ["LICENSE.txt", "README.md", "Cargo.lock", "Cargo.toml"]
  spec.files += Dir.glob("lib/**/*.rb")
  spec.files += Dir.glob("ext/**/*.{rs,toml,lock,rb}")
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }

  spec.require_paths = ["lib"]
  spec.extensions = ["ext/what_you_say/extconf.rb"]

  spec.metadata = {
    "allowed_push_host" => "https://rubygems.org",
    "funding_uri" => "https://github.com/sponsors/gjtorikian/",
    "source_code_uri" => "https://github.com/gjtorikian/what_you_say",
    "rubygems_mfa_required" => "true",
  }

  spec.add_dependency("rb_sys", "~> 0.9")

  spec.add_development_dependency("rake", "~> 13.0")
  spec.add_development_dependency("rake-compiler", "~> 1.2")
end
