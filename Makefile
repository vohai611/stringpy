# Minimal makefile for Sphinx documentation
#

# You can set these variables from the command line, and also
# from the environment for the first two.
SPHINXOPTS    ?=
SPHINXBUILD   ?= sphinx-build
SOURCEDIR     = docs
BUILDDIR      = build

# Put it first so that "make" without argument is like "make help".
help:
	@$(SPHINXBUILD) -M help "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)

.PHONY: help Makefile

# Catch-all target: route all unknown targets to Sphinx using the new
# "make mode" option.  $(O) is meant as a shortcut for $(SPHINXOPTS).
%: Makefile
	@$(SPHINXBUILD) -M $@ "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)

readme:
	@echo "Generating README.rst from README.md"
	@quarto render README.qmd 
	@mv README.rst docs/README.rst

test:
	@echo "Running tests"
	@pytest -rP
	@pytest --doctest-modules python/stringpy
rust:
	@echo "Build release"
	@maturin develop --release
cov: 
	@echo "test coveraage"
	@cargo llvm-cov --html
