require "./spec_helpers.cr"

describe "`and" do
  describe "new`" do
    new_ = run_with "new"
    it "does not accept non-ascii characters and dashes as project name" do
      (anders new_, "hello-world$!123 --package=bar").should be_failing_with exit_code 3
    end
    context "with sandbox" do
      project = "HelloWorld"
      package = "com.company.mypackage"
      keys = {"${project}" => project, "${package}" => package}
      substitute_context = ->(content : String) { content.gsub /\$\{\w+\}/, keys }

      it "successfully creates a project if the project name is valid" do
        sandboxed_anders new_, "#{project} --package #{package}" do |process, sandbox|
          process.should be_successful
          manifest = substitute_context.call MANIFEST
          main_java = substitute_context.call MAIN_JAVA
          sandbox.should have_file "#{project}/AndroidManifest.xml", with_content manifest
          sandbox.should have_file "#{project}/src/#{package.gsub '.', '/'}/#{project}.java", with_content main_java
        end
      end
    end
  end
end
