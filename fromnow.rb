# homebrew formula: https://github.com/Homebrew/brew
# note to self: wget -qO- url | shasum -a 256
# _ver='0.1.0';wget -qO- https://github.com/dashed/fromnow/releases/download/v$_ver/fromnow-v$_ver-x86_64-apple-darwin.tar.gz | shasum -a 256
# _ver='0.1.0';wget -qO- https://github.com/dashed/fromnow/releases/download/v$_ver/fromnow-v$_ver-i686-apple-darwin.tar.gz | shasum -a 256


require 'formula'
class Fromnow < Formula
  homepage 'https://github.com/dashed/fromnow'
  _ver = '0.8.0'
  version _ver

  if Hardware.is_64_bit?
    url "https://github.com/dashed/fromnow/releases/download/v#{_ver}/fromnow-v#{_ver}-x86_64-apple-darwin.tar.gz"
    sha256 '9b28f4ab8228d5ea944746c9f50d2a264c0989e330464ac8a436195fcb3a3376'
  else
    url "https://github.com/dashed/fromnow/releases/download/v#{_ver}/fromnow-v#{_ver}-i686-apple-darwin.tar.gz"
    sha256 '3b7486d5a24389e541c0d6f6a0a84d56cc668a56ffc36fc3155c40d295602d2d'
  end

  def install
    bin.install 'fromnow'
  end
end
