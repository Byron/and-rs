require "spec"
require "io"

struct ExitStatus
  getter status
  def initialize(@status : Int32)
  end
end

alias ExecutionResult = NamedTuple(result: Process::Status, output: MemoryIO, error: MemoryIO)

def process_details (p)
  <<-STRING
  
  
  DETAILS:
    --- STDOUT ---
    #{p[:output].to_s}
    --- STDERR ---
    #{p[:error].to_s}
  STRING
end

def and(runner, more_args)
  runner.call more_args
end

struct ProcessExpectation
  def initialize(@expected_value : ExitStatus)
  end

  def match(actual_value)
    actual_value[:result].exit_status == @expected_value
  end

  def failure_message(actual_value)
    "expected process to exit with: #{@expected_value.status} \n     got: #{actual_value[:result].exit_status}" + process_details actual_value
  end

  def negative_failure_message(actual_value)
    failure_message actual_value
  end
end

def exit_status(value)
  ExitStatus.new value
end

def be_failing_with(exit_code)
  ProcessExpectation.new exit_code
end

def be_successful()
  ProcessExpectation.new 0
end

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