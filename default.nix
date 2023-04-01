{ lib, fetchFromGithub, rustPlatform }:
rustPlatform.buildRustPackage rec {
    pname = "topaz";
    version = "a0.0.1";

    src = fetchFromGithub {
        owner = "topaz-lang";
        repo = pname;
        fetchSubmodules = true;
        rev = "6e75bb52df0f691a6789a2b70ac9edaf3f51e393";
        sha256 = lib.fakeSha256;
    };

    cargoLock = {
        lockFile = ./Cargo.lock;
    };

    cargoSha256 = lib.fakeSha256;

    meta = with lib; {
        description = "A programming language focused on performance, stability and simplicity.";
        homepage = "https://github.com/topaz-lang/topaz";
        license = licenses.gpl3;
        maintainers = [ maintainers.nothen maintainers.absoluty ];
    };
}
