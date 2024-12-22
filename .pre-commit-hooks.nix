{ git-hooks, system }: {
  pre-commit-check = git-hooks.lib.${system}.run {
    src = ./.;
    hooks = {
      fmt = {
        enable = true;
        name = "Formatting";
        entry = "make fmt";
        pass_filenames = false;
        files = "\\.rs$";
      };

      tests = {
        enable = true;
        name = "Tests";
        entry = "make test";
        pass_filenames = false;
        files = "\\.rs$";
      };

      lint = {
        enable = true;
        name = "Linting";
        entry = "make lint";
        pass_filenames = false;
        files = "\\.rs$";
      };

      nixfmt-classic.enable = true;
      statix.enable = true;

      checkmake = {
        enable = true;
        name = "Makefile Lint";
        entry = "checkmake";
        files = "Makefile$";
      };
    };
  };
}
