with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = [
  	    libxkbcommon

        # for druid
        cairo
        pango
        atk
        gdk-pixbuf
        gtk3-x11

        #needed for `shello` example
        #glib

  	    pkgconfig
  	    x11
];
  buildInputs = [
  	    xorg.libXi
  	    xorg.libXrandr
  	    xorg.libXcursor
];

  RUST_BACKTRACE = 1;


  }