require "./spec_helpers.cr"

describe "`and" do
  describe "new`" do
    new_ = run_with "new"
    it "does not accept non-ascii characters and dashes as project name" do
      (and new_, "hello-world$!123").should be_failing_with exit_status 3
    end
    
    it "successfully creates a project if the project name is valid" do
      (sandboxed_and new_, "HelloWorld").should be_successful
    end
  end
end
