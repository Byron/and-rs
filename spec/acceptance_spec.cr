require "./spec_helpers.cr"

describe "`and" do
  describe "new`" do
    new_ = run_with "new"
    it "does not accept non-ascii characters and dashes as project name" do
      (anders new_, "hello-world$!123 --package=bar").should be_failing_with exit_status 3
    end
    
    context "with sandbox" do
      project = "HelloWorld"
      package = "mypackage"
      
      it "successfully creates a project if the project name is valid" do
        sandboxed_anders new_, "#{project} --package #{package}" do |process, sandbox|
          process.should be_successful
          sandbox.should have_file "#{project}/AndroidManifest.xml"
        end
      end
    end
  end
end
