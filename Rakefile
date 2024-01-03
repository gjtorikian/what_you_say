# frozen_string_literal: true

if ENV.fetch("DEBUG", false)
  require "awesome_print"
  require "debug"
end

# Gem Spec
require "bundler"
GEMSPEC = Bundler.load_gemspec("what_you_say.gemspec")

# Packaging
require "rubygems/package_task"
gem_path = Gem::PackageTask.new(GEMSPEC).define
desc "Package the Ruby gem"
task "package" => [gem_path]
