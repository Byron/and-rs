require "spec"
require "io"

def run_with(args)
  ->(more_args : String) {
    output = MemoryIO.new()
    error = MemoryIO.new()
    {
      result: Process.run(
        command: ENV["EXECUTABLE"], 
        shell: false,
        args: "#{args} #{more_args}".split(' '),
        output: output, error: error, input: false),
      output: output,
      error: error
    }
  }
end

describe "`and` program" do
  describe "subcommands" do
    describe "new" do
      run = run_with "new"
      it "does not accept non-ascii characters and dashes as project name" do
        (run.call "hello-world$!123")[:result].exit_status.should eq 3
      end
    end
  end
end
