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
  getter result, output, error, invocation
  property sandbox_dir
  def initialize(@result : Process::Status, @output : MemoryIO, @error : MemoryIO, @sandbox_dir : String|Nil, @invocation : String)
  end
end

def process_details (process)
  <<-STRING
  
  DETAILS:
    >>> STDOUT
    #{process.output.to_s}
    <<< STDOUT
    >>> STDERR
    #{process.error.to_s}
    <<< STDERR
    #{if process.sandbox_dir
    "Program sandbox accessible at #{process.sandbox_dir}"
      end}
  STRING
end

def anders(runner, more_args)
  runner.call more_args, nil
end

def package_dir(package)
  "src/#{package.gsub '.', '/'}"
end

struct DirectoryExpectation
  enum Issue
    ContentMismatch
    Missing
  end
  
  def initialize(@expected_value : String, @expected_content : String|Nil = nil)
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
      <<-DETAILS
      expected sandbox to contain: #{@expected_value}
      See directory at #{actual_value} for more information
      DETAILS
    when Issue::ContentMismatch
      <<-DETAILS
      file #{@expected_value} did not have the correct content
      >>> ACTUAL
      #{@actual_content}
      <<< ACTUAL
      >>> EXPECTED
      #{@expected_content}
      <<< EXPECTED
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
    <<-DESCRIPTION
    CMD: #{actual_value.invocation}
    expected process to exit with: #{@expected_value.status}
         got: #{actual_value.result.exit_code}
         #{process_details actual_value}
    DESCRIPTION
  end

  def negative_failure_message(actual_value)
    failure_message actual_value
  end
end

def have_file(file, content : Nil|String = nil)
  DirectoryExpectation.new file, content
end

def have_dir(file)
  DirectoryExpectation.new file
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

def env_or_abort(variable)
  ENV[variable]?.try {|v| v} || ( puts "#{variable} environment variable must be set"; exit 5 )
end

def run_with(args)
  ->(more_args : String, chdir : String|Nil) {
    output = MemoryIO.new()
    error = MemoryIO.new()
    arguments = "#{args} #{more_args}"
    program = env_or_abort("EXECUTABLE")
    
    ExecutionResult.new(
      result: Process.run(
        command: program, 
        shell: false,
        args: arguments.split(' '),
        output: output, error: error, input: false,
        chdir: chdir
      ),
      invocation: "#{program} #{arguments}",
      output: output,
      error: error,
      sandbox_dir: nil
    )
  }
end

def with_project_and_then(runner, project, package, target)
  anders_new = run_with("new")
  ->(more_args : String, chdir : String|Nil) {
    process = anders_new.call "#{project} --package=#{package} --target=#{target}", chdir
    return process unless process.result.success?
    runner.call more_args, chdir
  }
end

