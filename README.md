# [cite_start]Eksperymentalne szacowanie złożoności obliczeniowej wybranych algorytmów sortowania – Sortowanie szybkie (Quick Sort) [cite: 2]

[cite_start]**Przedmiot:** Algorytmy i złożoność [cite: 1]  
[cite_start]**Autor:** Maksymilian Tym (73059) [cite: 3]  

---

## [cite_start]1. Krótki opis algorytmu [cite: 4]
[cite_start]Wybranym algorytmem do implementacji i analizy jest **Sortowanie szybkie (Quick Sort)**[cite: 5]. [cite_start]Jest to bardzo wydajny algorytm sortowania oparty na paradygmacie „dziel i zwyciężaj” (ang. *divide and conquer*)[cite: 6]. [cite_start]Działa on poprzez wybór jednego elementu z tablicy, nazywanego elementem osiowym (*pivotem*)[cite: 7]. [cite_start]W zaimplementowanej wersji jako pivot zawsze wybierany jest ostatni element aktualnie przetwarzanego fragmentu tablicy[cite: 8]. [cite_start]Następnie algorytm dzieli resztę elementów na dwie podtablice: elementy mniejsze lub równe pivotowi trafiają na lewo od niego, a większe na prawo[cite: 9]. [cite_start]Po tym procesie (zwanym partycjonowaniem) pivot znajduje się na swojej ostatecznej, posortowanej pozycji[cite: 10]. [cite_start]Cały proces jest powtarzany rekurencyjnie dla obu nowo powstałych podtablic, aż do momentu, gdy podtablice będą miały długość 1 lub 0 (co oznacza, że są już posortowane)[cite: 11].

## [cite_start]2. Specyfikacja algorytmu [cite: 12]
* [cite_start]**Dane wejściowe:** Nieposortowana ciągła struktura danych (w implementacji w języku Rust jest to modyfikowalny wycinek tablicy `&mut [i32]`) zawierająca liczby całkowite[cite: 13].
* [cite_start]**Dane wyjściowe:** Ta sama tablica, ale z elementami uporządkowanymi rosnąco (algorytm działa *in-place*, czyli nie wymaga tworzenia kopii tablicy)[cite: 14].
* [cite_start]**Kroki działania:** [cite: 15]
    1. [cite_start]Sprawdź, czy długość tablicy jest mniejsza lub równa 1. Jeśli tak, zakończ działanie dla tego fragmentu[cite: 16].
    2. [cite_start]Wybierz ostatni element jako pivot[cite: 17].
    3. [cite_start]Przesuń wszystkie elementy mniejsze lub równe pivotowi na początek tablicy, zliczając ich ilość[cite: 18].
    4. [cite_start]Zamień pivot miejscami z pierwszym elementem większym od niego, umieszczając go w docelowym miejscu[cite: 19].
    5. [cite_start]Wywołaj algorytm rekurencyjnie dla podtablicy po lewej stronie pivota[cite: 20].
    6. [cite_start]Wywołaj algorytm rekurencyjnie dla podtablicy po prawej stronie pivota[cite: 21].

## [cite_start]3. Teoretyczne oszacowanie złożoności [cite: 22]
* [cite_start]**Złożoność czasowa (przypadek średni i optymistyczny):** $O(n \log n)$[cite: 23]. [cite_start]Występuje, gdy pivot dzieli tablicę na dwie w miarę równe części[cite: 23]. [cite_start]Głębokość drzewa rekurencji wynosi wtedy $\log n$, a na każdym poziomie wykonujemy około $n$ operacji porównań[cite: 24].
* [cite_start]**Złożoność czasowa (przypadek pesymistyczny):** $O(n^2)$[cite: 25]. [cite_start]Zdarza się to w sytuacji, gdy pivot dzieli tablicę skrajnie nierównomiernie (np. gdy tablica wejściowa jest już posortowana rosnąco lub malejąco, a my wybieramy ostatni element)[cite: 25]. [cite_start]Wtedy jedna z podtablic ma zawsze $n-1$ elementów[cite: 26].
* [cite_start]**Złożoność pamięciowa:** $O(\log n)$[cite: 27]. [cite_start]Wynika ze zużycia pamięci na stos wywołań systemowych podczas rekurencji[cite: 27]. [cite_start]W przypadku pesymistycznym może wzrosnąć do $O(n)$[cite: 28].

## [cite_start]4. Wyniki badań eksperymentalnych [cite: 29]
[cite_start]Poniższe wyniki zostały osiągnięte na procesorze Intel(R) Core(TM) i7-8665U (8) @ 4.80 GHz na systemie operacyjnym opartym o kernel Linux[cite: 30].

| Rozmiar danych | Czas [ms] |
| :--- | :--- |
| 10 000 | 1 |
| 50 000 | 7 |
| 100 000 | 16 |
| 250 000 | 43 |
| 500 000 | 74 |
| 1 000 000 | 135 |
| 5 000 000 | 776 |
| 10 000 000 | 1535 |
![Wykres czasu działania algorytmu Quick Sort dla różnej ilości danych](static/time_of_sort.svg)
[cite_start]**Wnioski z eksperymentu:** Zgodnie z powyższym wykresem oraz tabelą danych, czas działania algorytmu Quick Sort rośnie proporcjonalnie do rozmiaru danych wejściowych w sposób nieliniowy, ale bardzo zbliżony do linii prostej[cite: 32]. [cite_start]Obserwowany kształt krzywej potwierdza teoretyczną średnią złożoność czasową $O(n \log n)$[cite: 33]. [cite_start]Język Rust (dzięki bezpośredniej kompilacji do kodu maszynowego i agresywnym optymalizacjom kompilatora dla profilu *release*) sprawia, że dla miliona elementów czas sortowania jest liczony w ułamkach sekund, co udowadnia wysoką skuteczność tego algorytmu w praktyce[cite: 34].