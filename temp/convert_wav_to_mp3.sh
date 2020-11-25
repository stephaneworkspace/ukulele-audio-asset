#!/bin/bash
for FILE in $PWD/*.wav ; do
  #OUTNAME=`basename "$FILE" .wav`.mp3
  OUTNAME=${FILE/.wav}.mp3
  ffmpeg -i "$FILE" -codec:a libmp3lame "$OUTNAME"
done
#ffmpeg -i "$file" -acodec libmp3lam -q:a "${file/%.wav/.mp3}"
#find -name "*.wav" -exec bash -c 'ffmpeg -i "{}" -codec:a libmp3lame -qscale:a 2 "${0/.wav}.mp3"' {} \;
