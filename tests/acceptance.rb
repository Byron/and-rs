
describe "and-rs" do
  context "given a project name with non-ascii characters" do
    it "should fail" do
      expect(true).to eq(false)
    end
  end
end