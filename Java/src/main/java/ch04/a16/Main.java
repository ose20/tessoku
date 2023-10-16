package ch04.a16;

import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();
        int[] a = new int[n+1];
        int[] b = new int[n+1];
        for (int i=2; i<=n; i++) a[i] = sc.nextInt();
        for (int i=3; i<=n; i++) b[i] = sc.nextInt();

        solve(n, a, b);
    }

    private static void solve(int n, int[] a, int[] b) {
        final int MAX = 1_000_000_000;
        int[] dp = new int[n+1];
        for (int i=0; i<=n; i++) dp[i] = MAX;

        dp[1] = 0;
        for (int i=2; i<=n; i++) {
            dp[i] = Math.min(dp[i], dp[i-1] + a[i]);
            if (i>=3) dp[i] = Math.min(dp[i], dp[i-2] + b[i]);
        }
        System.out.println(dp[n]);
    }
}
