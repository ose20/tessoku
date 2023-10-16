package ch03.A15;

import java.util.Collections;
import java.util.List;
import java.util.Scanner;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();
        List<Integer> a = IntStream.range(0, n).map(i -> sc.nextInt()).boxed().toList();

        solve(n, a);
    }

    private static void solve(int n, List<Integer> a) {
        List<Integer> sorted = a.stream().sorted().distinct().toList();

        System.out.println(a.stream()
                .map(ai -> Collections.binarySearch(sorted, ai) + 1)
                .map(String::valueOf)
                .collect(Collectors.joining(" "))
        );
    }
}
