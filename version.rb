# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://rubydoc.brew.sh/Formula
# PLEASE REMOVE ALL GENERATED COMMENTS BEFORE SUBMITTING YOUR PULL REQUEST!
class Version < Formula
  desc "A simple version manager for your projects."
  homepage "https://github.com/annie444/version"
  license "GPL-3.0-or-later"
  on_linux do
    on_arm do
      url "https://github.com/annie444/version/releases/download/v1.1.0/version-v1.1.0-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "d55504df373510060340b4ced46759856ea9515e689bf2b9fbd90dacc09b614b"
    end
    on_x86 do
      url "https://github.com/annie444/version/releases/download/v1.1.0/version-v1.1.0-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "17f1b3b0acd02c88a18befdac8d44d5a71fb6a259d40b9c417616f559ccee01a"
    end
  end
  on_macos do
    on_arm do
      url "https://github.com/annie444/version/releases/download/v1.1.0/version-v1.1.0-aarch64-apple-darwin.tar.gz"
      sha256 "4ef210de123e0e04125c69df786dcd7552a50c41d5a9f628d217cfe1b8454279"
    end
    on_x86 do
      url "https://github.com/annie444/version/releases/download/v1.1.0/version-v1.1.0-x86_64-apple-darwin.tar.gz"
      sha256 "3cc1da34fbfb27a6c3fa465cebc1ef250c9736f09fa5791a44bb353ece559de4"
    end
  end

  def install
    bin.install "version"
    man1.install Dir["manpages/*"]
    doc.install Dir["doc/*"]
  end
end
