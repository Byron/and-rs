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
      it "successfully creates a project if the project name is valid" do
        sandboxed_anders new_, "#{project} --package #{package}" do |process, sandbox|
          process.should be_successful
          manifest = MANIFEST.gsub /\$\{\w+\}/, {"${project}" => project, "${package}" => package}
          sandbox.should have_file "#{project}/AndroidManifest.xml", with_content manifest
        end
      end
    end
  end
end
