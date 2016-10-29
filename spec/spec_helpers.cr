require "spec"
require "io"
require "file_utils"
require "file"
require "./assets"

struct ExitCode
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
  enum Issue
    ContentMismatch
    Missing
  end
  
  def initialize(@expected_value : String, @expected_content : String|Nil)
    @issue = nil
    @actual_content = ""
  end
  
  def match(actual_value : String)
    path = File.join actual_value, @expected_value
    res = exists = File.exists? path
    @issue = Issue::Missing unless exists
    if exists
      @expected_content.try do |expected_content|
        @actual_content = File.read(path)
        content_matches = @actual_content == expected_content
        res = res && content_matches
        @issue = Issue::ContentMismatch unless content_matches
      end
    end
    res
  end
  
  def failure_message(actual_value)
    case @issue
    when Issue::Missing
      "expected sandbox to contain: #{@expected_value}"
    when Issue::ContentMismatch
      <<-DETAILS
      file #{@expected_value} did not have the correct content
      --- ACTUAL ---
      #{@actual_content}
      
      --- EXPECTED ---
      #{@expected_content}
      DETAILS
    end
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
  def initialize(@expected_value : ExitCode)
  end

  def match(actual_value)
    actual_value.result.exit_code == @expected_value.status
  end

  def failure_message(actual_value)
    "expected process to exit with: #{@expected_value.status} \n     got: #{actual_value.result.exit_code} #{process_details actual_value}"
  end

  def negative_failure_message(actual_value)
    failure_message actual_value
  end
end

def have_file(file, content : Nil|String = nil)
  DirectoryExpecation.new file, content
end

def with_content(content)
  content
end

def exit_code(value)
  ExitCode.new value
end

def be_failing_with(exit_code)
  ProcessExpectation.new exit_code
end

def be_successful()
  ProcessExpectation.new ExitCode.new 0
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
