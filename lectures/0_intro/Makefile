all: main cleanTex

main:
	cp themes/cob.tex conf/colors.tex
	pdflatex main.tex
	pdflatex main.tex
	mv main.pdf 0_intro.pdf

cob:
	cp themes/cob.tex conf/colors.tex
	pdflatex main.tex
	pdflatex main.tex
	mv main.pdf 0_intro.pdf

cog:
	cp themes/cog.tex conf/colors.tex
	pdflatex main.tex
	pdflatex main.tex
	mv main.pdf 0_intro.pdf

bow:
	cp themes/bow.tex conf/colors.tex
	pdflatex main.tex
	pdflatex main.tex
	mv main.pdf 0_intro.pdf

figures:
	./figures/generateFigures.sh

clean: cleanTex
	rm -rf *.pdf
	rm -rf figures/png/

cleanTex:
	rm -f *.aux *.log *.out *.blg *.bbl *.aux *.bbl *.blg *.log *.out *.toc \
			  *.thm *.lof *.tof *.ptc *.snm *.nav *.vrb

.PHONY: main clean cleanTex figures colors

