# Array Sum - Rust Versie

Dit is een alternatieve versie van het Array Sum practicum in Rust. We gebruiken hier Rayon in plaats van OpenMP om de oplossing over meerdere threads te laten draaien.
Om te bepalen of je voldoende van Rayon hebt begrepen, gaan we een 'eenvoudig' probleem proberen te parallelliseren. Het probleem is het volgende: tel alle waardes in een array bij elkaar op. Op zich niet lastig, maar aangezien we het over meerdere threads (cores) willen verdelen, komt er wat bij kijken. Om het probleem wat interessanter te maken, gaan we werken met grote arrays.

## Opgave

De file [main.rs](src/main.rs) is gegeven waarin een sequentiële variant van het programma is geïmplementeerd om waardes in een array bij elkaar te tellen. De arrays worden random in-memory gegenereerd. Compile en run met `cargo run`.

Pas de code aan om multi-threaded te draaien, en test deze met verschillende aantal threads (1, 2, 4, 8) en array sizes. Time de resultaten en maak een overzicht (spreadsheet) met de timing data per hoeveelheid threads van tests op arrays van 10K, 100K, 1M en 10M items. Als je wilt mag je een mooie grafiek maken om de performance winst per hoeveelheid threads duidelijk te maken, maar dat hoeft niet.

## Inleveren HPP Opdrachten
Voor de opdrachten van High Performance Programming lever je een verslag in, in PDF formaat.

Begin het verslag met:

- De titel van de opdracht;
- Je naam en studentnummer;
- Een link naar je GitHub Classroom repository met je werk.

- Lees de hele opdracht goed door, stel alvast vragen als iets niet duidelijk is
- Voor ieder deel / vraag:
  -  Vermeld het nummer van het deel of de vraag;
  -  Maak de opdracht en/of beantwoord de vraag;
  -  Kies code snippets en/of screenshots om je oplossing te laten zien;
  -  Beschrijf je oplossing beknopt, waarbij je vooral duidelijk maakt hoe je het hebt aangepakt.

Bewaar / exporteer je verslag als PDF en lever die in.
