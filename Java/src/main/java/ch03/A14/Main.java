package ch03.A14;

import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.Scanner;
import java.util.stream.IntStream;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();
        int k = sc.nextInt();

        int[] a = IntStream.range(0, n).map(i -> sc.nextInt()).toArray();
        int[] b = IntStream.range(0, n).map(i -> sc.nextInt()).toArray();
        int[] c = IntStream.range(0, n).map(i -> sc.nextInt()).toArray();
        int[] d = IntStream.range(0, n).map(i -> sc.nextInt()).toArray();

        solve(n, k, a, b, c, d);
    }

    private static void solve(int n, int k, int[] a, int[] b, int[] c, int[] d) {
        List<Integer> ab = Arrays.stream(a).boxed()
                .flatMap(ai -> Arrays.stream(b).mapToObj(bj -> ai + bj))
                .toList();

        List<Integer> cd = Arrays.stream(c).boxed()
                .flatMap(ci -> Arrays.stream(d).mapToObj(dj -> ci + dj))
                .sorted()
                .toList();

        System.out.println(
                ab.stream().anyMatch(x -> Collections.binarySearch(cd, k - x) >= 0) ? "Yes" : "No"
        );
    }

    private static <T> int lowerBound(List<? extends Comparable<? super T>> list, T key) {
        int pos = Collections.binarySearch(list, key);
        if (pos < 0) {
            pos = -(pos + 1);
        }
        return pos;
    }
}
