require "./spec_helpers.cr"

describe "`and` program" do
  describe "subcommands" do
    describe "new" do
      new_ = run_with "new"
      it "does not accept non-ascii characters and dashes as project name" do
        (and new_, "hello-world$!123").should be_failing_with exit_status 3
      end
    end
  end
end
