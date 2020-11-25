#!/bin/sh
find -name "*.wav" -exec bash 'ffmpeg -i "{}" -codec:a libmp3lame -qscale:a 2 "${0/.wav}.mp3' {} \;
