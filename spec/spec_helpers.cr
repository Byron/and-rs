require "spec"
require "io"
require "file_utils"
require "file"

struct ExitStatus
  getter status
  def initialize(@status : Int32)
  end
end

struct ExecutionResult
  getter result, output, error
  property sandbox_dir
  def initialize(@result : Process::Status, @output : MemoryIO, @error : MemoryIO, @sandbox_dir : String|Nil)
  end
end

def process_details (process)
  <<-STRING
  
  
  DETAILS:
    --- STDOUT ---
    #{process.output.to_s}
    --- STDERR ---
    #{process.error.to_s}
    #{if process.sandbox_dir
    "Program sandbox accessible at #{process.sandbox_dir}"
      end}
  STRING
end

def anders(runner, more_args)
  runner.call more_args, nil
end

struct DirectoryExpecation
  def initialize(@expected_value : String)
  end
  
  def match(actual_value : String)
    File.exists? File.join actual_value, @expected_value
  end
  
  def failure_message(actual_value)
    "expected sandbox to contain: #{@expected_value}"
  end

  def negative_failure_message(actual_value)
    failure_message actual_value
  end
end

def sandboxed_anders(runner, more_args, &block)
  tmpdir = `mktemp -d`
  tmpdir = tmpdir.strip
  process = runner.call more_args, tmpdir
  process.sandbox_dir = tmpdir
  
  yield process, tmpdir

  FileUtils.rm_r tmpdir
  process.sandbox_dir = nil
end

struct ProcessExpectation
  def initialize(@expected_value : ExitStatus)
  end

  def match(actual_value)
    actual_value.result.exit_status == @expected_value.status
  end

  def failure_message(actual_value)
    "expected process to exit with: #{@expected_value.status} \n     got: #{actual_value.result.exit_status} #{process_details actual_value}"
  end

  def negative_failure_message(actual_value)
    failure_message actual_value
  end
end

def have_file(file)
  DirectoryExpecation.new file
end

def exit_status(value)
  ExitStatus.new value
end

def be_failing_with(exit_code)
  ProcessExpectation.new exit_code
end

def be_successful()
  ProcessExpectation.new ExitStatus.new 0
end

def run_with(args)
  ->(more_args : String, chdir : String|Nil) {
    output = MemoryIO.new()
    error = MemoryIO.new()
    ExecutionResult.new(result: Process.run(
        command: ENV["EXECUTABLE"], 
        shell: false,
        args: "#{args} #{more_args}".split(' '),
        output: output, error: error, input: false,
        chdir: chdir),
      output: output,
      error: error,
      sandbox_dir: nil
    )
  }
end
