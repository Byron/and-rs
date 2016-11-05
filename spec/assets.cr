require "../src/lib/src/scaffolding/assets/manifest.xml.cr"
require "../src/lib/src/scaffolding/assets/main.java.cr"
require "../src/lib/src/scaffolding/assets/resource.xml.cr"

CONTEXT_JSON = <<-JSON
  "project": "${project}",
  "package": "${package}",
  "target": "${target}",
  "tasks":
JSON