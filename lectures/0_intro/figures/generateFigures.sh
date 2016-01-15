#!/bin/bash

figDir="figures/"
figDirLength=${#figDir}
skel="${figDir}figureSkel"
temp="temp"
tempTex="${temp}.tex"
tempPdf="${temp}.pdf"
resDir="${figDir}png/"

function fileNameOfPath {
  aux=""
  for i in $(echo $1 | tr "/" "\n")
  do
    aux=$1
  done
  return $aux
}

mkdir -p $resDir

printf "\033[34;4mBuilding all figures to ${resDir}.\033[0m\n"

files=`find $figDir -iname "*.tex"`

for path in $files
do
  fileClean=${path:$figDirLength:-4}
  filePng="${resDir}${fileClean}.png"
  # echo "skel: $skel"
  # echo "tempTex: $tempTex"
  # echo "tempPdf: $tempPdf"
  # echo "filePng: $filePng"
  printf "%45s... " "$filePng"
  sed "s|\input{.*}|\input{$path}|g" $skel > $tempTex
  pdflatex $tempTex > /dev/null
  convert -density 300 $tempPdf -quality 90 $filePng
  printf "done.\n" # > [$filePng]\n"
done

printf "%45s... " "cleaning things"
make cleanTex > /dev/null
rm -f $tempTex
rm -f $tempPdf
printf "done.\n"
printf "\033[34;4mAll good, exiting. Have a nice day.\033[0m\n"
