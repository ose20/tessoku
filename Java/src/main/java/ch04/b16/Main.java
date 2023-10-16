package ch04.b16;

import java.util.Scanner;
import java.util.stream.IntStream;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();
        int[] h = IntStream.range(0, n).map(i -> sc.nextInt()).toArray();

        solve(n, h);
    }

    private static void solve(int n, int[] h) {
        final int MAX = Integer.MAX_VALUE;
        int[] dp = new int[n+1];
        IntStream.range(0, n+1).forEach(i -> dp[i] = MAX);
        dp[1] = 0;

        for (int i=2; i<=n; i++) {
            dp[i] = Math.min(dp[i], dp[i-1] + Math.abs(h[i-1] - h[i-2]));
            if (i >= 3) dp[i] = Math.min(dp[i], dp[i-2] + Math.abs(h[i-1] - h[i-3]));
        }

        System.out.println(dp[n]);
    }
}
