package ch03.A11.A12;

import java.util.Arrays;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        int k = sc.nextInt();

        long[] a = new long[n];
        for (int i=0; i<n; i++) a[i] = sc.nextInt();

        solve(a, n, k);
    }

    private static void solve(long[] a, int size, int k) {
        int ng = 0;
        int ok = 1000000000;

        while (ng + 1 < ok) {
            int mid = (ok + ng) / 2;
            // mid 秒で k 枚印刷できるか
            long count = Arrays.stream(a).map(interval -> mid / interval).sum();
            if (count < k) ng = mid;
            else ok = mid;
        }

        System.out.println(ok);
    }
}
