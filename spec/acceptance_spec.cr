require "./spec_helpers.cr"

Spec.override_default_formatter Spec::VerboseFormatter.new
  
describe "`and" do
  project = "HelloWorld"
  package = "com.company.mypackage"
  target = env_or_abort("ANDROID_TARGET")
  
  describe "new`" do
    new_ = run_with "new"
    it "does not accept non-ascii characters and dashes as project name" do
      (anders new_, "hello-world$!123 --package=bar --target=foo").should be_failing_with exit_code 3
    end
    context "with sandbox" do
      substitution_keys = {"${project}" => project, "${package}" => package, "${target}" => target}
      substitute_context = ->(content : String) { content.gsub /\$\{\w+\}/, substitution_keys }

      it "successfully creates a project if the project name is valid" do
        sandboxed_anders new_, "#{project} --package #{package} --target=#{target}" do |process, sandbox|
          process.should be_successful
          manifest = substitute_context.call MANIFEST
          main_java = substitute_context.call MAIN_JAVA
          resource = substitute_context.call RESOURCE
          serialized_context = substitute_context.call CONTEXT_JSON
          
          ["obj", "lib", "bin"].each do |dir|
            sandbox.should have_dir "#{project}/#{dir}"
          end
          sandbox.should have_file "#{project}/AndroidManifest.xml", with_content manifest
          sandbox.should have_file "#{project}/src/#{package_dir package}/#{project}.java", with_content main_java
          sandbox.should have_file "#{project}/res/values/strings.xml", with_content resource
          sandbox.should have_file "#{project}/anders.json", with_content_matching serialized_context
        end
      end
    end
  end

  context = {project: project, package: package, target: target}
  compile = run_with "compile"
  describe "compile`" do
    it "should compile a project and generate bytecode and resources" do
      sandboxed_anders with_project_and_then(compile, **context), "--context=#{project}/anders.json" do |process, sandbox|
        process.should be_successful
        process.output.to_s.should contain "before compile"
        process.output.to_s.should contain "after compile"
        
        sandbox.should have_file "#{project}/src/#{package_dir package}/R.java"
        ["R$attr", "R$string", "R", project].each do |filename|
          sandbox.should have_file "#{project}/obj/#{package_dir package}/#{filename}.class"
        end
      end
    end
  end
  
  describe "package`" do
    package_cmd = run_with "package"
    it "should produce a signed, zipaligned package from compiled sources" do
      members = ["AndroidManifest.xml", "classes.dex", "resources.arsc"]
      sandboxed_anders with_project_and_then(compile, package_cmd, **context), "--context=#{project}" do |process, sandbox|
        process.should be_successful
        process.output.to_s.should contain "before package"
        process.output.to_s.should contain "after package"
        
        [".signed", ".unsigned", ""].each do |suffix|
          sandbox.should have_file "#{project}/bin/#{project}#{suffix}.apk", with_package_members members
        end
      end
    end
  end
end
