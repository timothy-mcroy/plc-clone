all: main cleanTex

main:
	pdflatex main.tex
	pdflatex main.tex
	mv main.pdf syllabus.pdf

clean: cleanTex
	rm -rf *.pdf *.tar.gz

cleanTex:
	rm -f *.aux *.log *.out *.blg *.bbl *.aux *.bbl *.blg *.log *.out *.toc *.thm *.lof *.tof *.ptc *.snm *.nav *.vrb

.PHONY: main clean
