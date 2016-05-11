
Make words that are anagrams or part anagrams of a main word.

Gather points for each word, and especially for finding the original main word.

So far, this is a text-only interface.

Usage: ``cargo run DICTIONARY``; the program takes a dictionary as the only
command line argument. It will pick a 7- or 8- letter word as the base
anagram. You have 60 seconds to guess as many words as possible.

Example
-------

::

    The word has 8 letters and it forms 39 words
    g n e g i t n e    you have 0 points and 60 seconds left
    ting
    +1 points for ting
     
    g n e e g n i t    you have 1 points and 42 seconds left
    geting
    +1 points for geting
    getingen
    +5 points for >>)getingen(<< !
    ^D
    Finished with 7 points.
    Target word: getingen
    Found words: geting, getingen, ting
    Missed: egen, eget, egg, eggen, ene, enen, enig, enigt, eten, gen, genen, geni, genien, geniet, get, geten, gin, ginge, igen, inge, ingen, inget, inne, inte, neg, nit, niten, teg, tegen, ten, tenen, tenn, tig, tigg, tingen, tinne
