#!/bin/bash

# Chillest
#   Songs To Dream To
#       Disc 1

ffmpeg -f lavfi -i "sine=frequency=220:duration=14" \
    -metadata ARTIST=Chillest -metadata ALBUM="Songs To Dream To" -metadata DATE=2021 -metadata DISCTOTAL=2 \
    -metadata DISCNUMBER=1 -metadata TRACK=1 -metadata TITLE="Lying There" \
    path_to_music/Chillest/Songs\ To\ Dream\ To/Disc\ 1/1.Lying\ There.flac

ffmpeg -f lavfi -i "sine=frequency=440:duration=12" \
    -metadata ARTIST=Chillest -metadata ALBUM="Songs To Dream To" -metadata DATE=2021 -metadata DISCTOTAL=2 \
    -metadata DISCNUMBER=1 -metadata TRACK=2 -metadata TITLE="Heavy Eyelids" \
    path_to_music/Chillest/Songs\ To\ Dream\ To/Disc\ 1/2.Heavy\ Eyelids.flac

ffmpeg -f lavfi -i "sine=frequency=660:duration=9" \
    -metadata ARTIST=Chillest -metadata ALBUM="Songs To Dream To" -metadata DATE=2021 -metadata DISCTOTAL=2 \
    -metadata DISCNUMBER=1 -metadata TRACK=3 -metadata TITLE="Dozing" \
    path_to_music/Chillest/Songs\ To\ Dream\ To/Disc\ 1/3.Dozing.flac

# Chillest
#   Songs To Dream To
#       Disc 2

ffmpeg -f lavfi -i "sine=frequency=330:duration=10" \
    -metadata ARTIST=Chillest -metadata ALBUM="Songs To Dream To" -metadata DATE=2021 -metadata DISCTOTAL=2 \
    -metadata DISCNUMBER=2 -metadata TRACK=1 -metadata TITLE="Enter the Dream" \
    path_to_music/Chillest/Songs\ To\ Dream\ To/Disc\ 2/1.Enter\ the\ Dream.flac

ffmpeg -f lavfi -i "sine=frequency=660:duration=20" \
    -metadata ARTIST=Chillest -metadata ALBUM="Songs To Dream To" -metadata DATE=2021 -metadata DISCTOTAL=2 \
    -metadata DISCNUMBER=2 -metadata TRACK=2 -metadata TITLE="The Adventure" \
    path_to_music/Chillest/Songs\ To\ Dream\ To/Disc\ 2/2.The\ Adventure.flac

ffmpeg -f lavfi -i "sine=frequency=440:duration=8" \
    -metadata ARTIST=Chillest -metadata ALBUM="Songs To Dream To" -metadata DATE=2021 -metadata DISCTOTAL=2 \
    -metadata DISCNUMBER=2 -metadata TRACK=3 -metadata TITLE="Sunlight" \
    path_to_music/Chillest/Songs\ To\ Dream\ To/Disc\ 2/3.Sunlight.flac

# The Rockers
#   Party Time

ffmpeg -f lavfi -i "sine=frequency=220:duration=16" \
    -metadata ARTIST="The Rockers" -metadata ALBUM="Party Time" -metadata DATE=2008 \
    -metadata TRACK=1 -metadata TITLE="Intro" \
    path_to_music/The\ Rockers/Party\ Time/1.Intro.flac

ffmpeg -f lavfi -i "sine=frequency=880:duration=26" \
    -metadata ARTIST="The Rockers" -metadata ALBUM="Party Time" -metadata DATE=2008 \
    -metadata TRACK=2 -metadata TITLE="The Hit" \
    path_to_music/The\ Rockers/Party\ Time/2.The\ Hit.flac

ffmpeg -f lavfi -i "sine=frequency=440:duration=11" \
    -metadata ARTIST="The Rockers" -metadata ALBUM="Party Time" -metadata DATE=2008 \
    -metadata TRACK=3 -metadata TITLE="Outro" \
    path_to_music/The\ Rockers/Party\ Time/3.Outro.flac

# The Rockers
#   Making A Point

ffmpeg -f lavfi -i "sine=frequency=220:duration=16" \
    -metadata ARTIST="The Rockers" -metadata ALBUM="Making A Point" -metadata DATE=2018 \
    -metadata TRACK=1 -metadata TITLE="Hear Me Out" \
    path_to_music/The\ Rockers/Making\ A\ Point/1.Hear_Me_Out.flac

ffmpeg -f lavfi -i "sine=frequency=880:duration=26" \
    -metadata ARTIST="The Rockers" -metadata ALBUM="Making A Point" -metadata DATE=2018 \
    -metadata TRACK=2 -metadata TITLE="This World" \
    path_to_music/The\ Rockers/Making\ A\ Point/2.This_World.flac

ffmpeg -f lavfi -i "sine=frequency=440:duration=11" \
    -metadata ARTIST="The Rockers" -metadata ALBUM="Making A Point" -metadata DATE=2018 \
    -metadata TRACK=3 -metadata TITLE="Thanks For Listening" \
    path_to_music/The\ Rockers/Making\ A\ Point/3.Thanks_For_Listening.flac
