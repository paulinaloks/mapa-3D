# Generator Map 3D

Program w Rust do generowania trójwymiarowych modeli terenu na podstawie map topograficznych.

## Funkcje

- Wczytywanie plików ASCII grid (.asc) zawierających dane wysokościowe
- Wykorzystanie interpolacji Lanczosa do wygładzania terenu
- Generowanie siatki 3D w formacie Wavefront OBJ
- Automatyczne otwieranie wygenerowanego modelu
- Shader Unity do kolorowania terenu na podstawie wysokości

## Wymagania

- Rust (najnowsza stabilna wersja)
- Unity (do wykorzystania shadera)
- Plik wejściowy w formacie .asc

## Instalacja

1. Sklonuj to repozytorium
2. Upewnij się, że masz zainstalowany Rust
3. Uruchom `cargo build --release` w katalogu projektu

## Użycie

1. Umieść plik .asc w katalogu projektu
2. Jeśli nazwa pliku jest inna niż "result.asc", zaktualizuj ją w `main.rs`
3. Uruchom program:
```bash
cargo run --release
```

Program:
- Wygeneruje plik modelu 3D (model.obj)
- Utworzy plik height.txt z informacjami o wysokościach terenu
- Automatycznie otworzy wygenerowany model

## Szczegóły Techniczne

Program wykorzystuje:
- Interpolację Lanczosa do wygładzania terenu
- Buforowane operacje I/O dla wydajności
- Równoległe przetwarzanie mapy wysokości
- Własny shader do wizualizacji

## Używanie Shadera

Dołączony shader HeightBasedColorWithLighting oferuje:
- Gradientowe kolorowanie bazujące na wysokości
- Podstawowe oświetlenie rozproszone
- Regulowane wartości min/max wysokości
- Dostosowywalne kolory

Aby użyć w Unity:
1. Zaimportuj shader
2. Stwórz nowy materiał używając tego shadera
3. Zastosuj do siatki terenu
4. Dostosuj właściwości w Inspektorze

## Licencja

Projekt dostępny na licencji MIT.
