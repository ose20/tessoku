package ch04.a19;

import java.util.Arrays;
import java.util.Scanner;
import java.util.stream.IntStream;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();
        int w = sc.nextInt();
        int[] ws = new int[n];
        long[] vs = new long[n];

        for (int i=0; i<n; i++) {
            ws[i] = sc.nextInt();
            vs[i] = sc.nextInt();
        }

        var dp = calcDp(n, w, ws, vs);

        System.out.println(
                Arrays.stream(dp[n], 0, w + 1).reduce(0L, Math::max)
        );

    }

    static long[][] calcDp(int n, int w, int[] ws, long[] vs) {
        // dp[i][j]: i番目までの数を使って重さをwにしたときの価値の最大値
        var dp = new long[n + 1][w + 1];
        for (int i = 0; i <= n; i++) for (int j = 0; j <= w; j++) dp[i][j] = -1;

        dp[0][0] = 0;

        for (int i = 1; i <= n; i++) for (int j = 0; j <= w; j++) {
            // i番目を使わない場合
            dp[i][j] = Math.max(dp[i][j], dp[i - 1][j]);
            // i番目を使う場合
            if (j - ws[i - 1] >= 0) dp[i][j] = Math.max(dp[i][j], dp[i - 1][j - ws[i - 1]] + vs[i - 1]);
        }

        return dp;
    }
}
