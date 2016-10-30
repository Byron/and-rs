require "./spec_helpers.cr"

describe "`and" do
  project = "HelloWorld"
  package = "com.company.mypackage"

  describe "new`" do
    new_ = run_with "new"
    it "does not accept non-ascii characters and dashes as project name" do
      (anders new_, "hello-world$!123 --package=bar").should be_failing_with exit_code 3
    end
    context "with sandbox" do
      substitution_keys = {"${project}" => project, "${package}" => package}
      substitute_context = ->(content : String) { content.gsub /\$\{\w+\}/, substitution_keys }

      it "successfully creates a project if the project name is valid" do
        sandboxed_anders new_, "#{project} --package #{package}" do |process, sandbox|
          process.should be_successful
          manifest = substitute_context.call MANIFEST
          main_java = substitute_context.call MAIN_JAVA
          resource = substitute_context.call RESOURCE
          serialized_context = substitute_context.call CONTEXT_JSON
          
          sandbox.should have_file "#{project}/AndroidManifest.xml", with_content manifest
          sandbox.should have_file "#{project}/src/#{package.gsub '.', '/'}/#{project}.java", with_content main_java
          sandbox.should have_file "#{project}/res/values/strings.xml", with_content resource
          sandbox.should have_file "#{project}/anders.json", with_content serialized_context
        end
      end
    end
  end
  describe "compile`" do
    compile = run_with "compile"
    context = {project: project, package: package}
    describe "compile`" do
      it "should compile a project and generate bytecode and resources" do
        sandboxed_anders with_project_and_then(compile, **context), "arguments" do |process, sandbox|
          process.should be_successful
        end
      end
    end
  end
end
