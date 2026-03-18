# Algorytm Dijkstry - Najkrótsze Ścieżki w Grafie

## Opis Algorytmu

Algorytm Dijkstry to algorytm służący do znalezienia najkrótszych ścieżek od wybranego wierzchołka do wszystkich pozostałych wierzchołków w grafie ważonym. Algorytm działa poprawnie dla grafów z wagami nieujemnymi.

Algorytm działa na zasadzie iteracyjnego wybierania wierzchołka o najmniejszej odległości i uaktualniania odległości jego sąsiadów. Proces powtarza się, aż wszystkie wierzchołki zostaną odwiedzone lub nie będzie możliwości znalezienia krótszych ścieżek.

## Złożoność Obliczeniowa

- **Złożoność czasowa**: O(V²), gdzie V to liczba wierzchołków
  - Dwu zagnieżdżone pętle: zewnętrzna iteruje V razy, wewnętrzna szuka minimum oraz aktualizuje odległości
  - Ta implementacja wykorzystuje naiwne podejście do wyszukiwania minimum

- **Złożoność pamięciowa**: O(V), gdzie przechowujemy:
  - Tablicę odległości dla każdego wierzchołka
  - Tablicę informacji o odwiedzonych wierzchołkach
  - Listę sąsiedztwa dla reprezentacji grafu

## Przykład Działania

### Dane Wejściowe
```
Enter number of vertices: 4
Enter number of edges: 5
Enter edges in format: [from] [to] [weight]
(Vertices are numbered from 0 to 3)
Edge 1: 0 1 2
Edge 2: 0 2 4
Edge 3: 1 2 1
Edge 4: 1 3 7
Edge 5: 2 3 2
Enter starting vertex: 0
```

### Wyniki
```
Shortest distances from vertex 0:
To vertex 0: 0
To vertex 1: 2
To vertex 2: 3
To vertex 3: 5
```

## Instrukcja Użytkownika - Wprowadzanie Danych

### Uruchomienie Programu
```bash
cargo run
```

### Kroki Wprowadzania Danych

1. **Liczba wierzchołków** - Podaj liczbę całkowitą reprezentującą ilość wierzchołków w grafie (np. 4)

2. **Liczba krawędzi** - Podaj liczbę całkowitą reprezentującą ilość krawędzi (np. 5)

3. **Krawędzie** - Dla każdej krawędzi wprowadź trzy liczby całkowite rozdzielone spacją:
   - Indeks wierzchołka początkowego (od 0)
   - Indeks wierzchołka końcowego (od 0)
   - Waga krawędzi (liczba całkowita dodatnia)
   
   Przykład: `0 1 5` oznacza krawędź od wierzchołka 0 do wierzchołka 1 z wagą 5

4. **Wierzchołek startowy** - Podaj indeks wierzchołka, od którego chcesz obliczyć najkrótsze ścieżki (np. 0)

### Ograniczenia
- Wierzchołki są indeksowane od 0 do (liczba wierzchołków - 1)
- Wszystkie wagi muszą być nieujemne
- Program akceptuje tylko poprawne indeksy wierzchołków

### Dane Wyjściowe

Program wyświetli tabelę zawierającą:
- "To vertex X: Y" - najkrótsza odległość do wierzchołka X wynosi Y
- "To vertex X: No path" - brak ścieżki do wierzchołka X (graf jest niespójny)

## Kompilacja

```bash
cargo build --release
```

## Pliki Projektu

- `src/main.rs` - Implementacja algorytmu Dijkstry w Rust
- `Cargo.toml` - Plik konfiguracyjny projektu
- `README.md` - Ten plik
