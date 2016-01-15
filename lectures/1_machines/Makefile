all: main cleanTex

main:
	cp themes/cob.tex conf/colors.tex
	pdflatex main.tex
	pdflatex main.tex
	mv main.pdf beamer.pdf

one:
	pdflatex main.tex
	mv main.pdf beamer.pdf

cob:
	cp themes/cob.tex conf/colors.tex
	pdflatex main.tex
	pdflatex main.tex
	mv main.pdf beamer.pdf

cog:
	cp themes/cog.tex conf/colors.tex
	pdflatex main.tex
	pdflatex main.tex
	mv main.pdf beamer.pdf

bow:
	cp themes/bow.tex conf/colors.tex
	pdflatex main.tex
	pdflatex main.tex
	mv main.pdf beamer.pdf

figures:
	./figures/generateFigures.sh

clean: cleanTex
	rm -rf *.pdf
	rm -rf figures/png/

cleanTex:
	rm -f *.aux *.log *.out *.blg *.bbl *.aux *.bbl *.blg *.log *.out *.toc \
			  *.thm *.lof *.tof *.ptc *.snm *.nav *.vrb

.PHONY: main clean cleaneTex figures colors

