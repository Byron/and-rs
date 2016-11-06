require "spec"
require "io"
require "file_utils"
require "file"
require "./assets"
require "zip-crystal/zip"

def travis()
  ENV["TRAVIS"]?
end

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
  "#{package.gsub '.', '/'}"
end

alias DirectoryExpectationValue = NamedTuple(content: String, partial: Bool)|Array(String)|Nil

struct DirectoryExpectation
  enum Issue
    ContentMismatch
    ContentPartialMismatch
    MemberMismatch
    FailedToParseZipFile
    Missing
    None
  end
  
  def initialize(@expected_value : String, @expected_content : DirectoryExpectationValue = nil)
    @issue = Issue::None
    @actual_content = ""
    @actual_zip_members = [] of String
  end
  
  def match(actual_value : String)
    path = File.join actual_value, @expected_value
    res = exists = File.exists? path
    @issue = Issue::Missing unless exists
    if exists
      case expected = @expected_content
      when Array(String)
        begin
          members = expected
          issue = Issue::None
          actual_members = [] of String
          Zip.read path do |zf|
            paths = zf.entries.map {|e| e.path}
            includes_all_members = (paths - members).size == paths.size - members.size
            res = res && includes_all_members
            actual_members = paths
            issue = Issue::MemberMismatch unless includes_all_members
          end
          @actual_zip_members = actual_members
          @issue = issue
        rescue
          res = false
          @issue = Issue::FailedToParseZipFile
        end
      when NamedTuple(content: String, partial: Bool)
        @actual_content = File.read(path)
        if expected[:partial]
          content_matches = @actual_content =~ Regex.new(expected[:content])
          @issue = Issue::ContentPartialMismatch unless content_matches
        else
          content_matches = @actual_content == expected[:content]
          @issue = Issue::ContentMismatch unless content_matches
        end
        res = res && content_matches
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
    when Issue::FailedToParseZipFile
      "file #{@expected_value} could not be parsed as zip file"
    when Issue::MemberMismatch
      <<-DETAILS
      zip file #{@expected_value} did not store all required members
      >>> ACTUAL MEMBERS
      #{@actual_zip_members.join('\n')}
      <<< ACTUAL MEMBERS
      >>> EXPECTED MEMBERS
      #{case ex = @expected_content
        when Array(String)
       ex.join('\n')
       end}
      <<<
      DETAILS
    when Issue::ContentPartialMismatch
      <<-DETAILS
      file #{@expected_value} content did not match.
      >>> CONTENT
      #{@actual_content}
      <<< CONTENT
      >>> DID NOT MATCH
      #{@expected_content.try do |ec|
        ec[:content] if ec.is_a? NamedTuple
      end}
      <<< DID NOT MATCH
      DETAILS
    when Issue::ContentMismatch
      <<-DETAILS
      file #{@expected_value} did not have the correct content
      >>> ACTUAL
      #{@actual_content}
      <<< ACTUAL
      >>> EXPECTED
      #{@expected_content.try do |ec|
        ec[:content] if ec.is_a? NamedTuple
      end}
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

struct ProcessOutputExpectation
  def initialize(@expected_value : String)
  end

  def match(actual_value)
    actual_value.output.to_s =~ Regex.new(@expected_value)
  end

  def failure_message(actual_value)
    <<-DESCRIPTION
    CMD: #{actual_value.invocation}
    >>> STANDARD OUTPUT
    #{actual_value.output.to_s}
    <<< STANDARD OUTPUT
    Did not contain #{@expected_value}
    DESCRIPTION
  end

  def negative_failure_message(actual_value)
    failure_message actual_value
  end
end

def have_file(file, content : DirectoryExpectationValue = nil)
  DirectoryExpectation.new file, content
end

def have_dir(file)
  DirectoryExpectation.new file
end


def with_content_matching(content)
  {content: content, partial: true}
end

def with_content(content)
  {content: content, partial: false}
end

def with_package_members(members : Array(String))
  members
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

def have_output_matching(content)
  ProcessOutputExpectation.new content
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

def with_project_and_then(*runners, project, package, target)
  anders_new = run_with("new")
  ->(more_args : String, chdir : String|Nil) {
    process = anders_new.call "#{project} --package=#{package} --target=#{target}", chdir
    return process unless process.result.success?
    runners.each do |runner|
      process = runner.call more_args, chdir
      return process unless process.result.success?
    end
    process
  }
end

