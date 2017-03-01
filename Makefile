.PHONY: all

SLIDE    = slide
HANDOUT  = handout
PDFNUP   = pdfnup
LATEX    = platex
DVIPDF   = dvipdfmx
IGNORES  = .aux .log .nav .out .snm .toc .pdf .dvi

all: $(SLIDE).pdf

clean:
	rm -f $(foreach suf,$(IGNORES),$(SLIDE)$(suf))

$(SLIDE).dvi: $(SLIDE).tex local.tex
	$(LATEX) $(SLIDE).tex

$(SLIDE).pdf: $(SLIDE).dvi
	$(DVIPDF) $<

$(HANDOUT).pdf: $(SLIDE).pdf
	$(PDFNUP) -o $@ --paper a4paper --nup 2x2 --landscape $<
