package ch04.a22;

import java.util.Arrays;
import java.util.Scanner;
import java.util.stream.IntStream;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();
        int[] a = IntStream.range(0, n-1).map(i -> sc.nextInt()).toArray();
        int[] b = IntStream.range(0, n-1).map(i -> sc.nextInt()).toArray();

        var dp = calcDp(n, a, b);
        System.out.println(dp[n]);
    }

    private static int[] calcDp(int n, int[] a, int[] b) {
        int[] dp = new int[n+1];
        boolean[] reachable = new boolean[n+1];
        reachable[1] = true;

        for (int i=1; i<n; i++) {
            if (!reachable[i]) continue;

            reachable[a[i-1]] = true;
            dp[a[i-1]] = Math.max(dp[a[i-1]], dp[i] + 100);

            reachable[b[i-1]] = true;
            dp[b[i-1]] = Math.max(dp[b[i-1]], dp[i] + 150);
        }

        return dp;
    }
}
